"use strict";

const LinkedList = require("../../00-linked-list/js/linkedList");

module.exports = class Queue {
  constructor() {
    this.linkedList = new LinkedList();
  }

  isEmpty() {
    return !this.linkedList.head;
  }

  peek() {
    if (!this.linkedList.head) {
      return;
    }
    return this.linkedList.head;
  }

  // add to the back of the queue
  enqueue(value) {
    this.linkedList.append(value);
  }

  // remove from the font of the queue
  dequeue() {
    this.linkedList.deleteHead();
  }

  toString() {
    return this.linkedList.toString();
  }
};
