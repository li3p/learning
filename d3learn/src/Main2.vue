<template>
  <div id="app">
    <svg id="dragdrop" width="600" height="200" style="background: beige">
      <line :x1="targetX" :y1="targetY" :x2="turningCX" :y2="turningCY" style="stroke:rgb(255,0,0);stroke-width:2"></line>
      <line :x1="sourceX" :y1="sourceY" :x2="turningCX" :y2="turningCY" style="stroke:rgb(255,0,0);stroke-width:2"></line>
      <circle id="target" :cx="targetX" :cy="targetY" r="8" fill="blue"/>
      <rect id="turning" :x="turningX" :y="turningY" :width="turningWidth" :height="turningWidth" fill="green"/>
      <circle id="source" :cx="sourceX" :cy="sourceY" r="20" fill="red"/>
    </svg>

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "App",
  computed: {
    // 计算属性的 getter
    turningCX: function () {
      // `this` 指向 vm 实例
      return this.turningX + this.turningWidth/2;
    },
    turningCY: function () {
      // `this` 指向 vm 实例
      return this.turningY + this.turningWidth/2 ;
    }
  },

  mounted: function () {
    let widget = undefined,
        color = undefined;

    let self = this;

    let drag = d3
        .drag()
        .on("start", function () {
          console.log('drag.start')
          color = d3.select(this).attr("fill");
          widget = d3.select(this).attr("fill", "lime");
        })
        .on("drag", function (evt) {
          var pt = d3.pointer(evt, d3.select(this).node());
          console.log('drag.draging', this.id, pt)
          if (this.id == 'source') {
            self.sourceX = pt[0];
            self.sourceY = pt[1];
          } else if(this.id == 'target'){
            self.targetX = pt[0];
            self.targetY = pt[1];
          } else if(this.id == 'turning') {
            self.turningX = pt[0];
            self.turningY = pt[1];
          }
          // widget.attr("cx", pt[0]).attr("cy", pt[1]);
        })
        .on("end", function () {
          widget.attr("fill", color);
          widget = undefined;
        });

    d3.select("#dragdrop").selectAll("circle").call(drag);
    d3.select("#dragdrop").selectAll("rect").call(drag);
  },

  data: () => ({
    targetX : 300,
    targetY : 100,
    sourceX : 500,
    sourceY : 100,
    turningX : 400,
    turningY : 100,
    turningWidth: 10,
  }),
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
