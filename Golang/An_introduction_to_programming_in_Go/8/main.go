package main

import "fmt"

func zero(x int){
	x =0
}

func zeroPointer(xPtr *int){
	*xPtr = 0
}

func one(xPtr *int){
	*xPtr = 1
}


func main(){
	x:=5
	zero(x)
	fmt.Println(x)
	zeroPointer(&x)
	fmt.Println(x)

	xPtr:=new(int)
	fmt.Println(xPtr)
	fmt.Println(*xPtr)
	one(xPtr)
	fmt.Println(xPtr)
	fmt.Println(*xPtr)
}
