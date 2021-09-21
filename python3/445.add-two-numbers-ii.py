#
# @lc app=leetcode id=445 lang=python3
#
# [445] Add Two Numbers II
#

from typing import Optional


class ListNode:
    def __init__(self, val: int = 0, next: Optional['ListNode'] = None):
        self.val = val
        self.next = next

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if l1 == None:
            return l2
        if l2 == None:
            return l1

        s1 = []
        s2 = []
        while l1:
            s1.append(l1.val)
            l1 = l1.next
        while l2:
            s2.append(l2.val)
            l2 = l2.next

        def insert(n1: list, n2: int):
            nonlocal curr
            sum = n1 + n2 + curr.val
            curr.val = sum % 10
            curr = ListNode(1 if sum >= 10 else 0, curr)

        curr = ListNode(0)
        while len(s1) > 0 and len(s2) > 0:
            insert(s1.pop(), s2.pop())

        while len(s1) > 0:
            insert(s1.pop(), 0)

        while len(s2) > 0:
            insert(0, s2.pop())

        return curr if curr.val > 0 else curr.next


# @lc code=end
