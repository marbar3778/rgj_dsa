package main

import "fmt"

type node struct {
	value    int
	next     *node
	previous *node
}

type List struct {
	head *node
	tail *node
}

func main() {
	dList := newList()

	dList.append(1)
	dList.append(2)
	dList.append(3)
	dList.printList()

	dList.prepend(0)
	dList.printList()

	// dList.delete(2)
	// dList.printList()

	// foundNode := dList.find(4)
	// if foundNode != nil {
	// 	fmt.Printf("Node: %+v \n", foundNode.value)
	// }

	dList.deleteTail()
	dList.printList()

	array := []int{5, 6, 7, 8, 9}
	dList.fromArray(array)
	dList.printList()

	// dList.deleteHead()
	// dList.printList()

	dList.printList()
	dList.reverse()
	dList.printList()

	fmt.Println(dList.toString())
}

func newList() *List {
	return &List{}
}

func (l *List) toString() []int {
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

func (l *List) printList() {
	list := l.head
	for list != nil {
		fmt.Printf("%+v -> ", list.value)
		list = list.next
	}
	fmt.Println()
}

func (l *List) append(value int) {
	newNode := &node{value, nil, nil}
	if l.head == nil {
		l.head = newNode
		l.tail = newNode
		return
	}
	mostRecent := l.tail
	mostRecent.next = newNode
	newNode.previous = mostRecent
	l.tail = newNode
}

func (l *List) prepend(value int) {
	newNode := &node{value, l.head, nil}
	l.head.previous = newNode
	l.head = newNode
	if l.tail == nil {
		l.tail = newNode
	}
}

func (l *List) delete(value int) {
	if l.head == nil {
		return
	}

	if l.head.value == value {
		l.head = l.head.next
		l.head.previous = nil
		return
	}

	currentNode := l.head
	for currentNode.next != nil {
		if currentNode.next.value == value {
			currentNode.next = currentNode.next.next
			currentNode.next.previous = currentNode
			return
		}
		currentNode = currentNode.next
	}
}

func (l *List) find(value int) *node {
	if l.head == nil {
		return nil
	}

	currentNode := l.head
	for currentNode.next != nil {
		if currentNode.value == value {
			return currentNode
		}
		currentNode = currentNode.next
	}
	fmt.Printf("Could not find a node with value: %v\n", value)
	return nil
}

func (l *List) deleteTail() {
	if l.head == nil {
		return
	}
	l.tail = l.tail.previous
	l.tail.next = nil
}

func (l *List) deleteHead() {
	if l.head == nil {
		return
	}
	l.head = l.head.next
	l.head.previous = nil
}

func (l *List) fromArray(array []int) {
	for _, value := range array {
		l.append(value)
	}
}

func (l *List) reverse() {
	if l.head == nil {
		return
	}

	currentNode := l.head
	var nextNode *node = nil
	var prevNode *node = nil

	for currentNode != nil {
		nextNode = currentNode.next
		prevNode = currentNode.previous

		currentNode.next = prevNode
		currentNode.previous = nextNode

		prevNode = currentNode
		currentNode = nextNode
	}
	l.tail = l.head
	l.head = prevNode
}
