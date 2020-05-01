package main

import "fmt"

type node struct {
	value int
	next  *node
}

type List struct {
	head *node
	tail *node
}

func NewList() *List {
	return &List{}
}

func (l *List) ToString() []int {
	if l.head == nil {
		return nil
	}
	values := []int{}
	list := l.head
	for list != nil {
		values = append(values, list.value)
		list = list.next
	}
	return values
}

func (l *List) PrintList() {
	if l == nil {
		return
	}
	list := l.head
	for list != nil {
		fmt.Printf("%+v -> ", list.value)
		list = list.next
	}
	fmt.Println()
}

func (l *List) Append(value int) {
	newNode := &node{value, nil}
	if l.head == nil {
		l.head = newNode
		l.tail = newNode
		return
	}
	mostRecent := l.tail
	mostRecent.next = newNode
	l.tail = newNode
}

func (l *List) Prepend(value int) {

	newNode := &node{value, l.head}
	l.head = newNode

	if l.tail == nil {
		l.tail = newNode
	}

}

func (l *List) Delete(valueDelete int) {
	if l == nil {
		return
	}

	if l.head.value == valueDelete {
		l.head = l.head.next
		return
	}

	currentNode := l.head

	for currentNode.next != nil {
		if currentNode.next.value == valueDelete {
			currentNode.next = currentNode.next.next
			return
		}
		currentNode = currentNode.next
	}
	return
}

// func find
func (l *List) DeleteTail() {
	if l == nil {
		return
	}

	currentNode := l.head

	for currentNode.next != nil {
		if currentNode.next.next == nil {
			currentNode.next = nil
		} else {
			currentNode = currentNode.next
		}
	}

	l.tail = currentNode
	return
}

func (l *List) DeleteHead() {
	if l == nil {
		return
	}
	l.head = l.head.next
	return
}

func (l *List) FromArray(array []int) {
	for _, value := range array {
		l.Append(value)
	}
}

func (l *List) Reverse() {
	if l.head == nil {
		return
	}
	currentNode := l.head
	var prevNode *node = nil
	var nextNode *node = nil

	for currentNode != nil {
		nextNode = currentNode.next
		currentNode.next = prevNode

		prevNode = currentNode
		currentNode = nextNode
	}

	l.tail = l.head
	l.head = prevNode
}
