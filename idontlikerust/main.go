package main

import (
	"bytes"
	"io"
	"net/http"
	"time"
)

var csv = bytes.NewBuffer(nil)
var lastFetch = time.Time{}

func main() {

	http.Handle("/", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		r.Header.Set("Access-Control-Allow-Origin", "*")
		r.Header.Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
		r.Header.Set("Access-Control-Allow-Headers",
			"Accept, Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization")

		if time.Since(lastFetch) > 1*time.Minute {
			resp, err := http.Get("https://framadate.org/exportcsv.php?poll=4eF5QE9cHUch9HF1")
			if err != nil {
				panic(err)
			}

			defer resp.Body.Close()

			csv = bytes.NewBuffer(nil)
			_, err = io.Copy(csv, resp.Body)
			if err != nil {
				panic(err)
			}

			lastFetch = time.Now()
		}

		w.Write(csv.Bytes())

	}))

	http.ListenAndServe(":8080", nil)

}
