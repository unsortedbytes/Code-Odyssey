package main

import (
	"fmt"
	"container/list"
)

func main(){
	var x list.List
	
	x.PushBack(1)
	x.PushBack(2)
	x.PushBack(3)
	x.PushBack(4)

	for e:= x.Front(); e!=nil;e=e.Next(){
		fmt.Println(e)
		fmt.Println(e.Value)
		fmt.Println(e.Value.(int))
	}
}
