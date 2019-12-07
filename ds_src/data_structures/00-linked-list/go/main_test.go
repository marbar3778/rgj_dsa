package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

func TestEmptyList(t *testing.T) {
	list := NewList()
	values := list.ToString()
	if values != nil {
		t.Error("Expected a empty list")
	}
}

func TestAppend(t *testing.T) {
	list := NewList()

	values := list.ToString()
	if values != nil {
		t.Error("Expected a empty list")
	}

	list.Append(1)
	list.Append(2)
	appendedValues := list.ToString()
	v := []int{1, 2}
	if !cmp.Equal(appendedValues, v) {
		t.Error("Expected [1 2]")
	}
}

func TestPrepend(t *testing.T) {
	list := NewList()

	list.Append(2)
	values := list.ToString()
	v := []int{2}
	if !cmp.Equal(values, v) {
		t.Error("Expected [2]")
	}

	list.Prepend(1)
	values2 := list.ToString()
	if !cmp.Equal([]int{1, 2}, values2) {
		t.Error("Expected [1 2]")
	}
}

func TestDelete(t *testing.T) {
	list := NewList()

	list.Append(1)
	list.Append(2)
	list.Append(3)
	list.Append(4)

	list.Delete(3)
	values := list.ToString()
	if !cmp.Equal([]int{1, 2, 4}, values) {
		t.Error("Expected [1 2 4]")
	}
}

func TestDeleteTail(t *testing.T) {
	list := NewList()

	list.Append(1)
	list.Append(2)
	list.Append(3)
	list.Append(4)

	list.DeleteTail()
	values := list.ToString()
	if !cmp.Equal([]int{1, 2, 3}, values) {
		t.Error("Expected [1 2 3]")
	}
}

func TestDeleteHead(t *testing.T) {
	list := NewList()

	list.Append(1)
	list.Append(2)
	list.Append(3)
	list.Append(4)

	list.DeleteHead()
	values := list.ToString()
	if !cmp.Equal([]int{2, 3, 4}, values) {
		t.Error("Expected [2 3 4]")
	}
}

func TestFromArray(t *testing.T) {
	list := NewList()
	array := []int{5, 6, 7, 8}

	list.FromArray(array)
	values := list.ToString()
	if !cmp.Equal(array, values) {
		t.Error("Expected array")
	}
}

func TestReverse(t *testing.T) {
	list := NewList()

	array := []int{5, 6, 7, 8}
	list.FromArray(array)

	list.Reverse()

	values := list.ToString()
	if !cmp.Equal([]int{8, 7, 6, 5}, values) {
		t.Error("Expected reverse array")
	}
}
