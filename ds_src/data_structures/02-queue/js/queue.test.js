"use strict";
const Queue = require("./queue");

describe("Queue Tests", () => {
  it("should check if the queue is empty", () => {
    const q = new Queue();

    expect(q.isEmpty()).toBe(true);

    q.enqueue(1);
    expect(q.isEmpty()).toBe(false);
    expect(q.toString()).toBe("1");
  });
  it("should enqueue a new item", () => {
    const q = new Queue();

    q.enqueue(1);

    expect(q.isEmpty()).toBe(false);
    expect(q.toString()).toBe("1");

    q.enqueue(2);
    expect(q.toString()).toBe("1,2");
  });
  it("should dequeue a item from the Queue", () => {
    const q = new Queue();

    q.enqueue(1);
    q.enqueue(2);
    expect(q.toString()).toBe("1,2");

    q.dequeue();
    expect(q.toString()).toBe("2");
  });
  it("should check what is the head", () => {
    const q = new Queue();

    q.enqueue(1);
    q.enqueue(2);
    expect(q.toString()).toBe("1,2");

    const item = q.peek();
    expect(item.value).toBe(1);
  });
});
