package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCheckQueue(t *testing.T) {
	q := newQueue()

	assert.Equal(t, q.isEmpty(), true)

	q.Enqueue(2)
	assert.Equal(t, q.isEmpty(), false)
	q.peek()
}
