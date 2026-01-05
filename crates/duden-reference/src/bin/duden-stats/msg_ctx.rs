use std::{borrow::Cow, fmt::Display, sync::mpsc};

#[derive(Debug, Clone, PartialEq)]
pub enum MsgCtx<'a> {
    TopLevel,
    Sub {
        parent: Box<MsgCtx<'a>>,
        name: Cow<'a, str>,
    },
}
impl<'a> Display for MsgCtx<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopLevel => Ok(()),
            Self::Sub { parent, name } if **parent == Self::TopLevel => f.write_str(name.as_ref()),
            Self::Sub { parent, name } => {
                Display::fmt(parent, f)?;
                f.write_str(" / ")?;
                f.write_str(name.as_ref())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MSender {
    ctx: MsgCtx<'static>,
    sender: std::sync::mpsc::Sender<(MsgCtx<'static>, Cow<'static, str>)>,
}

impl MSender {
    pub fn channel() -> (
        Self,
        std::sync::mpsc::Receiver<(MsgCtx<'static>, Cow<'static, str>)>,
    ) {
        let (tx, rx) = std::sync::mpsc::channel();
        let ctx = MsgCtx::TopLevel;
        (Self { ctx, sender: tx }, rx)
    }
    pub fn scope(&self, name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        MSender {
            ctx: MsgCtx::Sub {
                parent: self.ctx.clone().into(),
                name,
            },
            sender: self.sender.clone(),
        }
    }
    pub fn c_send(
        &self,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<(), mpsc::SendError<(MsgCtx<'static>, Cow<'static, str>)>> {
        self.sender.send((self.ctx.clone(), message.into()))
    }
    pub fn send(&self, message: impl Into<Cow<'static, str>>) {
        let _ = self.c_send(message);
    }
}
