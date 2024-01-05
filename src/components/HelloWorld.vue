<template>
  <div class="hello">
    <h2> {{ result }}</h2>
    <p>
      <a href="https://cli.vuejs.org" target="_blank" rel="noopener">vue-cli documentation</a>.
    </p>
    <ul>
      <li><a href="https://github.com/vuejs/vue-cli/tree/dev/packages/%40vue/cli-plugin-babel" target="_blank" rel="noopener">babel</a></li>
      <li><a href="https://github.com/vuejs/vue-cli/tree/dev/packages/%40vue/cli-plugin-eslint" target="_blank" rel="noopener">eslint</a></li>
    </ul>
    <h2> {{ sum }}</h2>
    <ul>
      <li><a href="https://vuejs.org" target="_blank" rel="noopener">Core Docs</a></li>
      <li><a href="https://forum.vuejs.org" target="_blank" rel="noopener">Forum</a></li>
      <li><a href="https://chat.vuejs.org" target="_blank" rel="noopener">Community Chat</a></li>
      <li><a href="https://twitter.com/vuejs" target="_blank" rel="noopener">Twitter</a></li>
      <li><a href="https://news.vuejs.org" target="_blank" rel="noopener">News</a></li>
    </ul>
    <h3>Ecosystem</h3>
    <ul>
      <li><a href="https://router.vuejs.org" target="_blank" rel="noopener">vue-router</a></li>
      <li><a href="https://vuex.vuejs.org" target="_blank" rel="noopener">vuex</a></li>
      <li><a href="https://github.com/vuejs/vue-devtools#vue-devtools" target="_blank" rel="noopener">vue-devtools</a></li>
      <li><a href="https://vue-loader.vuejs.org" target="_blank" rel="noopener">vue-loader</a></li>
      <li><a href="https://github.com/vuejs/awesome-vue" target="_blank" rel="noopener">awesome-vue</a></li>
    </ul>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';
// import { load } from 'wasm-import';

const result = ref('');
const sum = ref(0);

const extractDataFromAddress = (address, memory) =>  {
  // Get the memory buffer of the WebAssembly instance
  const memoryBuffer = new Uint8Array(memory);

  // Find the end index of the null-terminated string
  let endIndex = address;
  while (memoryBuffer[endIndex] !== 0) {
    endIndex++;
  }

  // Extract the string from the memory buffer
  const extractedString = String.fromCharCode.apply(null, memoryBuffer.subarray(address, endIndex));

  return extractedString;
};

onMounted(async () => {
  // await load('main.wasm');
  // Load the WebAssembly module
  fetch("main.wasm")
      .then(response => response.arrayBuffer())
      .then(bytes => WebAssembly.instantiate(bytes, {
    env: {
      printString: (line) => {
        console.log(`The line is ${line}`);
      },
      print: (result) => {
        console.log(`The result is ${result}`);
        sum.value = result;
    }
    }})).then(results => {
        const { add, repeat, printZigit } = results.instance.exports;
        console.log(add, repeat, results.instance.exports.memory, printZigit);
        result.value = extractDataFromAddress(printZigit(), results.instance.exports.memory.buffer);

        add(1, 2);
        repeat([100, 101]);
      })
      .catch(error => console.error(error));

  // const typedArray = new Uint8Array(fetch("main.wasm"));

  // WebAssembly.instantiate(typedArray, {
  //   env: {
  //     print: (result) => { console.log(`The result is ${result}`); }
  //   }}).then(result => {
  //   const add = result.instance.exports.add;
  //   add(1, 2);
  // });
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
