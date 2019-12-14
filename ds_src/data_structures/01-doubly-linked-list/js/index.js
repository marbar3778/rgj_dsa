"use strict";

class Node {
  constructor(value, next = null, previous = null) {
    this.value = value;
    this.next = next;
    this.previous = previous;
  }
}

module.exports = class DoublyList {
  constructor() {
    this.head = null;
    this.tail = null;
  }

  append(value) {
    let newNode = new Node(value);
    if (!this.head) {
      this.head = newNode;
      this.tail = newNode;
      return this;
    }
    this.tail.next = newNode;
    newNode.previous = this.tail;
    this.tail = newNode;
    return this;
  }

  prepend(value) {
    let newNode = new Node(value, this.head);
    this.head.previous = newNode;
    this.head = newNode;
    if (!this.tail) {
      this.tail = newNode;
    }
    return this;
  }

  delete(value) {
    if (!this.head) {
      return;
    }

    if (this.head.value === value) {
      this.head = this.head.next;
      return;
    }

    let currentNode = this.head;
    if (currentNode != null) {
      while (currentNode.next) {
        if (currentNode.next.value == value) {
          currentNode.next = currentNode.next.next;
          currentNode.next.previous = currentNode;
        } else {
          currentNode = currentNode.next;
        }
      }
    }
    if (this.tail.value == value) {
      this.tail = currentNode;
    }
    return;
  }

  deleteHead() {
    if (!this.head) {
      return;
    }
    if (this.head.next) {
      this.head = this.head.next;
      this.head.previous = null;
      return;
    }
    this.head = null;
  }

  deleteTail() {
    if (this.head == this.tail) {
      this.head = null;
      this.tail = null;
      return;
    }
    let currentNode = this.head;
    while (currentNode.next) {
      if (!currentNode.next.next) {
        currentNode.next = null;
      } else {
        currentNode = currentNode.next;
      }
      this.tail = currentNode;
    }
  }

  find(value) {
    if (!this.head) {
      return;
    }
    let currentNode = this.head;
    while (currentNode) {
      if (currentNode.value == value) {
        return currentNode;
      }
      currentNode = currentNode.next;
    }
  }

  fromArray(array) {
    array.forEach(n => this.append(n));
  }

  toArray() {
    const nodes = [];

    let currentNode = this.head;
    while (currentNode) {
      nodes.push(currentNode);
      currentNode = currentNode.next;
    }
    return nodes;
  }

  toString() {
    // console.log(this.toArray().map(n => n.value));
    return this.toArray()
      .map(node => node.value)
      .toString();
  }

  reverse() {
    let currentNode = this.head;
    let prevNode = null;
    let nextNode = null;

    while (currentNode) {
      nextNode = currentNode.next;
      prevNode = currentNode.previous;

      currentNode.next = prevNode;
      currentNode.previous = nextNode;

      prevNode = currentNode;
      currentNode = nextNode;
    }
    this.tail = this.head;
    this.head = prevNode;
  }
};
