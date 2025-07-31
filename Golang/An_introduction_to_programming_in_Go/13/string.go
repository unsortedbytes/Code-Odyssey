package main

import (
	"fmt"
	"strings"
)

func main(){
	// Contains -> check the 2nd paramete is persent or not return bool
	fmt.Println(
		strings.Contains("test", "ex"),
		strings.Contains("text","ex"),
	)

	// Count -> the number and return the value
	fmt.Println(
		strings.Count("text","t"),
		strings.Count("aksfj ihhfiahifdfhad ffahdifhhshdfu", "f"),
	)

	// HasSuffix -> check the last part only
	fmt.Println(
		strings.HasSuffix("aditya", "ya"),
		strings.HasSuffix("Aditya", "Ad"),
	)

	// HasPrefix -> check the first part only
	fmt.Println(
		strings.HasPrefix("Aditya", "ad"),
		strings.HasPrefix("aditya", "ad"),
	)

	// Index
	fmt.Println(
		strings.Index("Aditya", "A"),
		strings.Index("Aditya", "d"),
		strings.Index("Aditya", "a"),
		strings.Index("Aditya", "y"),
	)

	// Join
	fmt.Println(
		strings.Join([]string{"a","b"}, "YYYYY"),
		strings.Join([]string{"Aditya", "Kumar"}, " "),
	)

	// Repeat
	fmt.Println(
		strings.Repeat("Mahi ", 5),
		strings.Repeat("Kohli ", 10),
	)

	// Replace
	fmt.Println(
		strings.Replace("Aditya Kumar", "a", "X", 10 ),
		strings.Replace("ijs sio osof ooajoidf aiodfj addifjajsiodfj aidjfijaif","i", "X", 10),
	)

	// Split
	fmt.Println(
		strings.Split("Adityakumar", "k"),
	)

	// ToLower
	fmt.Println(
		strings.ToLower("Text"),
	)

	// ToUpper
	fmt.Println(
		strings.ToUpper("test"),
	)


	// Convert string to slice of byte 
	arr := []byte("test")
	fmt.Println(arr)

	str := string([]byte{'t','e','x','t'})
	fmt.Println(str)
}
