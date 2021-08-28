const rust = import('./pkg')
rust.then(m => {
  m.say_hello_from_rust()
  const sum = m.sum(3,4);
  console.log('sum: ', sum);
}).catch(console.error);
