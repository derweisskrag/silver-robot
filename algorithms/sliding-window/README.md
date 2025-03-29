# Sliding window

## Old code

```js

const foo = (array, number) => {

  let tempSum = 0;
  let maxSum = 0;

  for(let i = 0; i < number; i++){
    tempSum += array[i];
  }

  tempSum = maxSum;

  for(let i = number; i < array.length; i++){
    tempSum += array[i] - array[i - number];
    maxSum = Math.max(maxSum, tempSum);
  }

  return maxSum;
};
```

### Good pairs

```js
function countGood(nums, k){
    const result = new Map();
    let goodPairs = 0;
    let count = 0;
    for(let i = 0; i < nums.length; i++){
        if(!result[nums[i]]){
            result[nums[i]] = 1;
        } else{
            result[nums[i]]++;
        }
    }

    for(let i = 0; i < nums.length; i++){
        if(result[nums[i]]){
            if((i * (i - 1) / 2) >= k){
                count++;
            }
            if(goodPairs >= k){
                count++;
            }

            goodPairs += --result[nums[i]];
        }
    }

    return count;
}
```

#### Anothe example

```js
var countGood = function(nums, k) {
  let n = nums.length, count = {}, goodPairs = 0, ans = 0;
  for (let j = 0, i = 0; j < n; j++) {
    goodPairs += count[nums[j]] || 0;
    count[nums[j]] = (count[nums[j]] || 0) + 1;
    while (goodPairs >= k) {
      goodPairs -= --count[nums[i]];
      i++;
    }
    ans += i;
  }
  return ans;
};
```