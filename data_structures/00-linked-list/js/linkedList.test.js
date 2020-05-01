const LinkedList = require("./linkedList");

describe("Linked List", () => {
  it("should crete a new Linked List", () => {
    const linkedList = new LinkedList();
    expect(linkedList.toString()).toBe("");
  });
  it("should append node to linked list", () => {
    const linkedList = new LinkedList();

    expect(linkedList.head).toBeNull();
    expect(linkedList.tail).toBeNull();

    linkedList.append(1);
    linkedList.append(2);

    expect(linkedList.toString()).toBe("1,2");
  });
  it("should prepend node to linked list", () => {
    const linkedList = new LinkedList();

    expect(linkedList.head).toBeNull();
    expect(linkedList.tail).toBeNull();

    linkedList.append(1);
    linkedList.append(2);

    expect(linkedList.toString()).toBe("1,2");

    linkedList.prepend(3);

    expect(linkedList.toString()).toBe("3,1,2");
  });
  it("should delete node by value from linked list", () => {
    const linkedList = new LinkedList();

    expect(linkedList.head).toBeNull();
    expect(linkedList.tail).toBeNull();

    linkedList.append(1);
    linkedList.append(2);

    expect(linkedList.toString()).toBe("1,2");

    const deleted = linkedList.delete(2);
    expect(deleted.value).toBe(2);
    expect(linkedList.toString()).toBe("1");

    linkedList.append(3);
    expect(linkedList.toString()).toBe("1,3");
    const deleted1 = linkedList.delete(1);
    expect(linkedList.toString()).toBe("3");
    expect(deleted1.value).toBe(1);
  });
  it("should delete linked list tail", () => {
    const linkedList = new LinkedList();

    expect(linkedList.head).toBeNull();
    expect(linkedList.tail).toBeNull();

    linkedList.append(1);
    linkedList.append(2);

    expect(linkedList.toString()).toBe("1,2");

    const tail = linkedList.deleteTail();
    expect(tail.value).toBe(2);

    expect(linkedList.toString()).toBe("1");
  });
  it("should delete linked list head", () => {
    const linkedList = new LinkedList();

    expect(linkedList.head).toBeNull();
    expect(linkedList.tail).toBeNull();

    linkedList.append(1);
    linkedList.append(2);

    expect(linkedList.toString()).toBe("1,2");

    const deletedHead = linkedList.deleteHead();
    expect(deletedHead.value).toBe(1);

    expect(linkedList.toString()).toBe("2");
  });
  // it("should be possible to store objects in the list and to print them out", () => {});

  it("should make a linkedList from an array", () => {
    const linkedList = new LinkedList();

    const array = [1, 2, 3, 4, 5];

    linkedList.fromArray(array);

    expect(linkedList.toString()).toBe("1,2,3,4,5");
  });
});
