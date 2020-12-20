<template>
  <div id="app">
    <circle-mark :init-points="points[0]" @dragged="onDragged"/>

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import CircleMark from "@/components/CircleMark";

export default {
  name: "App",

  components: {
    CircleMark,
  },
  
  methods: {
    onDragged: function (points) {
      console.log("after drag", points);
    }
  },

  mounted: function () {
  },

  data: function () {
    return {
      points: [
        [[200,50]]
      ]
    }
  },
};
</script>

<style>
#app {
  font-family: "Open Sans", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #282f36;
  margin-top: 30px;
}
</style>
