package config

import(
	"github.com/jinzhu/gorm"
	"github.com/jinzhu/gorm/gorm/dialects/mysql"
)

var(
	db *gorm.DB
)

func Connect(){
	d, err := gorm.Open("mysql", "user:username/tablename?charset=utf8&parseTime=True&loc=Local")
	
	if err!= nil{
		panic(err)
	}
	db =d 
}

func GetDB() *gorm.DB{
	return db
}
