const DoublyList = require("./index");

describe("Doubly LinkedList", () => {
  it("should create a empty list", () => {
    const list = new DoublyList();

    expect(list.toString()).toBe("");
    expect(list.head).toBeNull();
    expect(list.tail).toBeNull();
  });
  it("should append a Node to the list", () => {
    const list = new DoublyList();

    expect(list.toString()).toBe("");
    expect(list.head).toBeNull();
    expect(list.tail).toBeNull();

    list.append(1);
    expect(list.toString()).toBe("1");
    expect(list.head.next).toBeNull();

    list.append(2);
    expect(list.toString()).toBe("1,2");
    expect(list.head.next.value).toBe(2);
    expect(list.tail.previous.value).toBe(1);
  });
  it("should prepend a node to a list", () => {
    const list = new DoublyList();
    list.append(1);
    list.append(2);

    expect(list.toString()).toBe("1,2");

    list.prepend(3);
    expect(list.toString()).toBe("3,1,2");
    expect(list.head.next.value).toBe(1);
    expect(list.head.next.previous.value).toBe(3);
    expect(list.tail.previous.value).toBe(1);
  });
  it("should delete a specific node", () => {
    const list = new DoublyList();
    expect(list.toString()).toBe("");

    list.append(1);
    list.append(2);
    list.append(3);
    expect(list.toString()).toBe("1,2,3");

    list.delete(1);
    expect(list.toString()).toBe("2,3");
    list.prepend(1);
    expect(list.toString()).toBe("1,2,3");
    list.delete(1);
    expect(list.toString()).toBe("2,3");
  });
  it("should populate the list from an array of values", () => {
    const list = new DoublyList();
    const array = [1, 2, 3, 4, 5];

    list.fromArray(array);
    expect(list.toString()).toBe("1,2,3,4,5");
  });
  it("should delete the head", () => {
    const list = new DoublyList();
    list.append(1);
    list.deleteHead();

    expect(list.toString()).toBe("");
    list.append(1);
    list.append(2);
    list.deleteHead();

    expect(list.toString()).toBe("2");
    expect(list.head.previous).toBeNull();
  });
  it("should delete the tail", () => {
    const list = new DoublyList();
    list.append(1);
    list.append(2);
    list.append(3);

    expect(list.toString()).toBe("1,2,3");
    list.deleteTail();
    expect(list.toString()).toBe("1,2");
  });
  it("should find a specific node", () => {
    const list = new DoublyList();
    const array = [1, 2, 3, 4, 5];

    list.fromArray(array);
    expect(list.toString()).toBe("1,2,3,4,5");

    const node2 = list.find(2);
    expect(node2.value).toBe(2);

    const node4 = list.find(4);
    expect(node4.value).toBe(4);
  });
});
