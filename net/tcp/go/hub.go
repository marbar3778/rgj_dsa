package main

import (
	"strings"
)

type hub struct {
	channels        map[string]*channel // chat rooms
	clients         map[string]*client  // map of connected users
	commands        chan commands
	deregistrations chan *client
	registrations   chan *client
}

func newHub() hub {
	return hub{
		channels:        make(map[string]*channel),
		clients:         make(map[string]*client),
		commands:        make(chan commands),
		deregistrations: make(chan *client),
		registrations:   make(chan *client),
	}
}

func (h *hub) run() {

	for {
		select {
		case client := <-h.registrations:
			h.register(client)
		case client := <-h.deregistrations:
			h.deregister(client)
		case cmd := <-h.commands:
			switch cmd.id {
			case JOIN:
				h.joinChannel(cmd.sender, cmd.recipient)
			case LEAVE:
				h.leaveChannel(cmd.sender, cmd.recipient)
			case MSG:
				h.message(cmd.sender, cmd.recipient, cmd.body)
			case USRS:
				h.listUsers(cmd.sender)
			case CHNS:
				h.listChannels(cmd.sender)
			default:
				panic("nothing")
			}
		}
	}
}

func (h *hub) register(c *client) {
	// check if the username already exists
	if _, exists := h.clients[c.username]; exists {
		c.username = ""
		c.conn.Write([]byte("err username taken\n"))
	} else {
		h.clients[c.username] = c
		c.conn.Write([]byte("OK\n"))
	}
}

func (h *hub) deregister(c *client) {
	// check if the username already is on the hub, if so remove it
	if _, exists := h.clients[c.username]; exists {
		delete(h.clients, c.username)

		for _, channel := range h.channels {
			delete(channel.clients, c)
		}
	}
}

func (h *hub) joinChannel(s string, r string) {
	if client, ok := h.clients[s]; ok {
		if channel, ok := h.channels[r]; ok {
			// Channel exists, join
			channel.clients[client] = true
		} else {
			// Channel doesn't exists, create and join
			h.channels[r] = newChannel(r)
			h.channels[r].clients[client] = true
		}
	}
}

func (h *hub) leaveChannel(s string, r string) {
	if client, ok := h.clients[s]; ok {
		if channel, ok := h.channels[r]; ok {
			delete(channel.clients, client)
		}
	}
}

func (h *hub) message(u string, r string, m []byte) {
	if sender, ok := h.clients[u]; ok {
		switch r[0] {
		case '#':
			if channel, ok := h.channels[r]; ok {
				if _, ok := channel.clients[sender]; ok {
					channel.broadcast(sender.username, m)
				}
			}
		case '@':
			if user, ok := h.clients[r]; ok {
				user.conn.Write(append(m, '\n'))
			}
		}
	}
}

func (h *hub) listUsers(s string) {
	if client, ok := h.clients[s]; ok {
		var names []string

		if len(h.channels) == 0 {
			client.conn.Write([]byte("ERR no channels found\n"))
		}

		for u := range h.clients {
			names = append(names, u)
		}

		resp := strings.Join(names, ", ")

		client.conn.Write([]byte(resp + "\n"))
	}
}

func (h *hub) listChannels(s string) {
	if client, ok := h.clients[s]; ok {
		var names []string

		if len(h.channels) == 0 {
			client.conn.Write([]byte("ERR no channels found\n"))
		}

		for _, channel := range h.channels {
			names = append(names, channel.name)
		}

		resp := strings.Join(names, ", ")

		client.conn.Write([]byte(resp + "\n"))
	}
}
