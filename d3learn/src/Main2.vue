<template>
  <div id="app">
    <svg id="dragdrop" width="600" height="200">
      <circle cx="100" cy="100" r="20" fill="red" />
      <rect x="300" y="100" width="10" height="10" r="20" fill="green" />
      <circle cx="500" cy="100" r="20" fill="blue" />
    </svg>

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "App",
  mounted: function () {
    var widget = undefined,
      color = undefined;

    var drag = d3
      .drag()
      .on("start", function () {
        console.log('drag.start')
        color = d3.select(this).attr("fill");
        widget = d3.select(this).attr("fill", "lime");
      })
      .on("drag", function (evt) {
        var pt = d3.pointer(evt, d3.select(this).node());
        console.log('drag.draging', this, pt)
        widget.attr("cx", pt[0]).attr("cy", pt[1]);
      })
      .on("end", function () {
        widget.attr("fill", color);
        widget = undefined;
      });

      d3.select("#dragdrop").selectAll("circle").call(drag);
  },

  data: () => ({}),
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
