const rust = import("./pkg");
rust.then(module => {
  console.log('max month is ', module.calculate_max([
    {month: 'jan', value: 100},
    {month: 'feb', value: 1000},
    {month: 'mar', value: 104},
    {month: 'apr', value: 1400},
  ]))
})
