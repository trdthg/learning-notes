 
function a(a, b) {
    console.log(a)
    console.log(b)
}

a(1)

/**
 *
 * @param {Function[]} fns
 */
function run(fns) {
  fns.forEach((x, i) => {
    console.log(i, "???");
    x()

  })
}

run([
  () => {
    console.log(1)
  },
  () => {
    console.log(2)
  }
])
