/*
 * @lc app=leetcode id=146 lang=typescript
 *
 * [146] LRU Cache
 */

// @lc code=start
class LNode {
    constructor(
        public key: number = 0,
        public val: number = 0,
        public prev: LNode | undefined = undefined,
        public next: LNode | undefined = undefined
    ) {}
}

class LRUCache {
    private readonly _head = new LNode();

    private readonly _tail = new LNode();

    private readonly _map = new Map<number, LNode>();

    constructor(private readonly _capacity: number) {
        this._head.next = this._tail;
        this._tail.prev = this._head;
    }

    get(key: number): number {
        const v = this._map.get(key);
        if (v === undefined) return -1;
        this._remove(v);
        this._insert(v);
        return v.val;
    }

    put(key: number, value: number): void {
        let v = this._map.get(key);
        if (v !== undefined) {
            v.val = value;
            this._remove(v);
            this._insert(v);
        } else {
            if (this._map.size === this._capacity) {
                const last = this._tail.prev;
                this._remove(last);
                this._map.delete(last.key);
            }
            const node = new LNode(key, value);
            this._insert(node);
            this._map.set(key, node);
        }
    }

    private _remove(node: LNode): void {
        const prev = node.prev;
        const next = node.next;
        prev.next = next;
        next.prev = prev;
    }

    private _insert(node: LNode): void {
        const prev = this._head;
        const next = prev.next;
        prev.next = node;
        next.prev = node;
        node.prev = prev;
        node.next = next;
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */
// @lc code=end
