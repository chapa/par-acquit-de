package main

import (
	"io"
	"net/http"
)

func main() {

	http.Handle("/", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		resp, err := http.Get("https://framadate.org/exportcsv.php?poll=4eF5QE9cHUch9HF1")
		if err != nil {
			panic(err)
		}

		defer resp.Body.Close()

		_, err = io.Copy(w, resp.Body)
		if err != nil {
			panic(err)
		}
	}))

	http.ListenAndServe(":8080", nil)

}
