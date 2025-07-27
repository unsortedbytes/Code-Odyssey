package main

import "fmt"
import "os"

func main(){
	// fuction second part defer panic ans second part 
	// using the defer
	defer first()
	second()
	f,_ := os.Open("file.txt")
	defer f.Close()


	// Panic 

	// panic("PANIC")
	//str:=recover()
	//fmt.Println!(str)
	// It will never print as exectuion stop 

	// for it we have to define the function
	//
	defer func (){
		str:=recover()
		fmt.Println(str)
	}()
	// panic("PANIC")

	x,y:=half(34)
	a,b:=half(45)
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(x)
	fmt.Println(y)


}

func half(a int) (int, bool){
	half:= a/2
	if half %2 ==0 {
		return half, true
	}
	return half, false
}

func first(){
	fmt.Println("First")
}

func second(){
	fmt.Println("Second")
}
