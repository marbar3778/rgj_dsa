const TrieNode = require("./trieNode");

const HEAD_CHAR = "*";

module.exports = class trie {
  constructor() {
    this.head = new TrieNode(HEAD_CHAR);
  }

  // Takes a string returns a trie
  Add(word) {
    const characters = Array.from(word);
    let currNode = this.head;
    for (let i = 0; i < characters.length; i++) {
      const isComp = charIndex === characters - 1;
      currNode = currNode.addChild(characters[charIndex], isComp);
    }
    return this;
  }

  // Takes a string returns a trie
  Delete(word) {
    const depthFirstDelete = (currNode, charIndex = 0) => {
      if (charIndex >= word.length) {
        return;
      }

      const character = word[charIndex];
      const nextNode = currNode.getChild(character);

      if (nextNode == null) {
        return;
      }

      depthFirstDelete(currNode, charIndex + 1);

      if (charIndex === word.length - 1) {
        nextNode.isCompleteWord = false;
      }

      // childNode is deleted only if:
      // - childNode has NO children
      // - childNode.isCompleteWord === false
      currentNode.removeChild(character);
    };

    // Start depth-first deletion from the head node.
    depthFirstDelete(this.head);

    return this;
  }

  //  Takes a string returns string[]
  SuggestNextChar(word) {
    const lastChar = this.getLastCharacterNode(word);

    if (!lastChar) {
      return null;
    }

    return lastChar.suggestChildren();
  }

  // String, return bool
  DoesWordExist(word) {
    const lastChar = this.getLastCharacterNode(word);

    return !!lastChar && lastChar.isComp;
  }

  // String, return TrieNode
  GetLastCharNode(word) {
    const chars = Array.from(word);
    let currNode = this.head;
    for (let i = 0; i < chars.length; i++) {
      if (!currNode.hasChild(chars[i])) {
        return null;
      }
      currNode = currNode.getChild(chars[i]);
    }
    return currNode;
  }
};
