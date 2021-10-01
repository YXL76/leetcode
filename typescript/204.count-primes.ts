/*
 * @lc app=leetcode id=204 lang=typescript
 *
 * [204] Count Primes
 */

// @lc code=start
function countPrimes(n: number): number {
    if (n <= 2) return 0;
    let count = 2;
    const isPrime = new Array(n).fill(true);
    for (let i = 2; i < n; i++) {
        if (!isPrime[i]) continue;
        for (let j = i * i; j < n; j += i)
            if (isPrime[j]) {
                isPrime[j] = false;
                ++count;
            }
    }
    return n - count;
}
// @lc code=end
