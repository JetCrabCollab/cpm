const _ = require('lodash');

console.log('Hello from CPM-managed JavaScript project!');
console.log('Lodash version:', _.VERSION);

// Example using lodash
const numbers = [1, 2, 3, 4, 5];
const doubled = _.map(numbers, n => n * 2);
console.log('Original:', numbers);
console.log('Doubled:', doubled);
