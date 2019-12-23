package main

import (
	"container/list"
)

func main() {
	q := newQueue()
	q.Enqueue(2)
	q.Enqueue(3)
	// q.toString()
	// fmt.Println(q.listy.value)
}

type Queue struct {
	listy *list.List
}

func newQueue() Queue {
	newList := list.New()
	return Queue{newList}
}

func (q Queue) Enqueue(value int) {
	q.listy.PushBack(value)
}

func (q Queue) dequeue() interface{} {
	firstItem := q.listy.Front()

	dequeuedItem := q.listy.Remove(firstItem)

	return dequeuedItem
}

func (q Queue) peek() interface{} {
	return q.listy.Front()
}

func (q Queue) isEmpty() bool {
	if q.listy.Front() == nil {
		return true
	}
	return false
}

func (q Queue) toString() string {
	q.listy.
}
