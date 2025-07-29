package main

import ("fmt";"math")

func distance(x1, y1, x2 , y2 float64) float64{
	a:=x1-x2
	b:=y1-y2
	return  math.Sqrt(a*a + b*b)
}

func rectangleArea(x1, y1, x2, y2 float64) float64{
	l := distance(x1, y1, x1, y2)
	w := distance(x1, y1, x2, y1)
	return l*w
}

func circleArea(x, y, r float64) float64{
	return math.Pi * r*r
}

func circleAreaUpdated (c Circle) float64{
	return math.Pi * c.r * c.r
}

func circleUpdated (c *Circle) {
	c.x=4
	c.y=5
	c.r=10
}

// type Circle struct {
// 	x float64
// 	y float64
// 	r float64
// }
type Circle struct{
	x, y, r float64
}
func main(){

//	var c Circle
	// c := new(Circle)
	// rturn pointer
	c1:=Circle{x:0, y:0, r:6}
	c:= Circle{0, 0 , 5}
	circleUpdated(&c)

	fmt.Println(c.x, c.y, c.r)
	c.x = 10
	c.y = 5

	fmt.Println(c1.x, c1.y, c1.r)
	fmt.Println(c.x, c.y, c.r)
	fmt.Println("Area of cricle is :", circleAreaUpdated(c))











	var rx1, ry1 float64 = 0, 0
	var rx2 , ry2 float64 = 10, 10
	var cx, cy, cr float64 = 0, 0, 5

	fmt.Println(rectangleArea(rx1, ry1, rx2, ry2))
	fmt.Println(circleArea(cx, cy, cr))

}
