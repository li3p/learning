<template>
  <div id="app">
    <svg id="dragdrop" width="600" height="200" style="background: beige">
      <defs>
        <marker id="arrowhead" markerWidth="6" markerHeight="4"
                refX="0" refY="2" orient="auto">
          <polygon points="0 0, 6 2, 0 4"/>
        </marker>
      </defs>
      <line id="mainLine" marker-end="url(#arrowhead)"
            :x1="startX" :y1="startY"
            :x2="targetX" :y2="targetY"
            style="stroke:rgb(255,0,0);stroke-width:2"/>
      <line v-show="hasControl" :x1="sourceX" :y1="sourceY"
            :x2="controlX" :y2="controlY" style="stroke:rgb(255,0,0);stroke-width:2"/>
      <circle id="target" :cx="targetX" :cy="targetY" r="12" opacity="0"/>
      <rect v-show="hasControl" id="control" :x="controlX-controlWidth/2"
            :y="controlY-controlWidth/2" :width="controlWidth"
            :height="controlWidth" rx="2" ry="2" fill="green"/>
      <circle id="source" :cx="sourceX" :cy="sourceY" r="20" fill="red"/>
    </svg>

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "Mark",

  props: {
    initPoints: Array
  },
  watch: {
    initPoints: function (val) {
      console.log('initPoints changed')
    },
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
          } else if (this.id == 'target') {
            self.targetX = pt[0];
            self.targetY = pt[1];
          } else if (this.id == 'control') {
            self.controlX = pt[0];
            self.controlY = pt[1];
          }
          // widget.attr("cx", pt[0]).attr("cy", pt[1]);
        })
        .on("end", function () {
          widget.attr("fill", color);
          widget = undefined;
        });

    d3.select("#control").on("contextmenu", function (evt) {
      console.log(evt, evt.shiftKey)
      evt.preventDefault();
      if (evt.shiftKey) {
        self.hasControl = false;
      }
    })

    d3.select("#mainLine").on("dblclick", function (evt) {
      let pt = d3.pointer(evt, d3.select(this).node());
      self.controlX = pt[0];
      self.controlY = pt[1];
      self.hasControl = true;

    }).on("mouseover", function (evt) {
      console.log(evt, evt.shiftKey)
      if (!self.hasControl) {
        d3.select(this).style("cursor", "col-resize");
      }
    }).on("mouseout", function (evt) {
      console.log(evt, evt.shiftKey)
      d3.select(this).style("cursor", "default");
    });

    d3.select("#dragdrop").selectAll("circle").call(drag);
    d3.select("#dragdrop").selectAll("rect").call(drag);

    d3.selectAll("#source,#target,#control")
        .on("mouseover", function (evt) {
          d3.select(this).style("cursor", "grab");
        })
        .on("mouseout", function (evt) {
          d3.select(this).style("cursor", "default");
        });
  },

  computed: {
    startX: function () {
      return this.hasControl ? this.controlX : this.sourceX;
    },
    startY: function () {
      return this.hasControl ? this.controlY : this.sourceY;
    }
  },

  data: function () {
    return {
      targetX: this.initPoints[0] ? this.initPoints[0][0] : 100,
      targetY: this.initPoints[0] ? this.initPoints[0][1] : 100,
      sourceX: this.initPoints[1] ? this.initPoints[1][0] : 100,
      sourceY: this.initPoints[1] ? this.initPoints[1][1] : 100,
      controlX: this.initPoints[2] ? this.initPoints[2][0] : 100,
      controlY: this.initPoints[2] ? this.initPoints[2][1] : 100,
      controlWidth: 10,
      hasControl: this.initPoints[2],
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
