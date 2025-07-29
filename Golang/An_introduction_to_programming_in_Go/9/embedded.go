package main

import "fmt"

type Shape interface{
	area() float64
}

type Person struct {
	Name string
}

func (p *Person) Talk(){
	fmt.Println("Hi, my name is ", p.Name)
}

type Andriod struct {
	Person
	Model string
}

func main(){
	p :=Person{"Aditya"}
	a:=Andriod{p, "latest"}

	aditya:=Andriod{Person{"aditya"}, "new"}
	fmt.Println(a)
	a.Person.Talk()
	
	fmt.Println(aditya)
	aditya.Talk()
	
}
