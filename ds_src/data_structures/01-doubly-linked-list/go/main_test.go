package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/stretchr/testify/assert"
)

func TestEmptyList(t *testing.T) {
	list := newList()
	values := list.toString()
	if values != nil {
		t.Error("Expected a empty list")
	}
}

func TestAppend(t *testing.T) {
	list := newList()

	values := list.toString()
	if values != nil {
		t.Error("Expected a empty list")
	}

	list.append(1)
	list.append(2)
	list.append(3)
	appendedValues := list.toString()
	v := []int{1, 2, 3}
	if !assert.Equal(t, appendedValues, v) {
		t.Error("Expected [1 2 3]")
	}
}

func TestPrepend(t *testing.T) {
	list := newList()

	list.append(2)

	appendedValues := list.toString()
	if !assert.Equal(t, appendedValues, []int{2}) {
		t.Error("Expected [2]")
	}

	list.prepend(1)
	values2 := list.toString()
	if !cmp.Equal([]int{1, 2}, values2) {
		t.Error("Expected [1 2]")
	}
}

func TestDelete(t *testing.T) {
	list := newList()

	list.append(1)
	list.append(2)
	list.append(3)
	list.append(4)

	list.delete(3)
	values := list.toString()
	if !cmp.Equal([]int{1, 2, 4}, values) {
		t.Error("Expected [1 2 4]")
	}
}

func TestDeleteTail(t *testing.T) {
	list := newList()

	list.append(1)
	list.append(2)
	list.append(3)
	list.append(4)

	list.deleteTail()
	values := list.toString()
	if !cmp.Equal([]int{1, 2, 3}, values) {
		t.Error("Expected [1 2 3]")
	}
}

func TestDeleteHead(t *testing.T) {
	list := newList()

	list.append(1)
	list.append(2)
	list.append(3)
	list.append(4)

	list.deleteHead()
	values := list.toString()
	if !cmp.Equal([]int{2, 3, 4}, values) {
		t.Error("Expected [2 3 4]")
	}
}

func TestFromArray(t *testing.T) {
	list := newList()
	array := []int{5, 6, 7, 8}

	list.fromArray(array)
	values := list.toString()

	if !cmp.Equal(array, values) {
		t.Error("Expected array")
	}
}

func TestReverse(t *testing.T) {
	list := newList()

	array := []int{5, 6, 7, 8}
	list.fromArray(array)

	list.reverse()
	values := list.toString()
	if !cmp.Equal([]int{8, 7, 6, 5}, values) {
		t.Error("Expected reverse array")
	}
}
