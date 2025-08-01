package main

import (
	"encoding/gob"
	"fmt"
	"net"
	"time"
)

func server(){
	//	listen on the port
	ln, err := net.Listen("tcp", ":9989")
	if err!= nil{
		fmt.Println(err)
		return
	}
	for {
		// accept the connection
		c, err := ln.Accept()
		if err != nil{
			fmt.Println(err)
			continue
		}
		// handle the connection
		go handleServerConnection(c)
	}
}

func handleServerConnection(c net.Conn){
	// recive the message 
	var msg string
	err := gob.NewDecoder(c).Decode(&msg)
	if err != nil{
		fmt.Println(err)
	}else{
		fmt.Println("Recived", msg)
	}

	c.Close()
}


func client(){
	// connect to the server
	c, err := net.Dial("tcp", "127.0.0.1:9989")
	if err!= nil {
		fmt.Println(err)
		return
	}
	// send the message 

	msg:= "Hello Aditya"
	fmt.Println("Sending in .....", msg)
	err = gob.NewEncoder(c).Encode(msg)
	if err != nil{
		fmt.Println(err)
	}
	c.Close()
}


func main(){
	go server()
	time.Sleep(100*time.Millisecond)
	go client()
	var input string
	fmt.Scanln(&input)
}
