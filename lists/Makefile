lists: wordlist-orig-no-trailing-spaces.txt

wordlist-original.txt:
	curl "https://gist.githubusercontent.com/MarvinJWendt/2f4f4154b8ae218600eb091a5706b5f4/raw/36b70dd6be330aa61cd4d4cdfda6234dcb0b8784/wordlist-german.txt" -o wordlist-original.txt

wordlist-orig-no-trailing-spaces.txt: wordlist-original.txt
	sed -E "s/ $$//g" wordlist-original.txt > wordlist-orig-no-trailing-spaces.txt

