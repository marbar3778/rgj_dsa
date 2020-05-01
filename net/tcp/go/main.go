package main

import (
	"log"
	"net"
)

type ID int

const (
	REG ID = iota
	JOIN
	LEAVE
	MSG
	CHNS
	USRS
)

// command represents what the sender would like to do
type commands struct {
	id        ID
	recipient string
	sender    string
	body      []byte
}

func main() {
	ln, err := net.Listen("tcp", ":8081")
	if err != nil {
		log.Printf("%v", err)
	}

	hub := newHub()
	go hub.run()

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("%v", err)
		}

		c := newClient(
			conn,
			hub.commands,
			hub.registrations,
			hub.deregistrations,
		)
		go c.read()
	}
}
