#
# @lc app=leetcode id=206 lang=python3
#
# [206] Reverse Linked List
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
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        curr = head
        while True:
            next = curr.next
            if next is None:
                return head
            next_next = next.next
            curr.next = next_next
            next.next = head
            head = next

# @lc code=end
