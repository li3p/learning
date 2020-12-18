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
        .on("drag", function (evt) {
          let pt = d3.pointer(evt, d3.select(this).node());
          if (this.id == 'source') {
            self.sourceX = pt[0];
            self.sourceY = pt[1];
          } else if (this.id == 'target') {
            self.targetX = pt[0];
            self.targetY = pt[1];
          } else if (this.id == 'control') {

            if (evt.sourceEvent.shiftKey) {
              let angle = self.getAngle(self.targetX, self.targetY, pt[0], pt[1])
              console.log("angle: ", Math.floor(angle / 45), evt.sourceEvent.shiftKey)

              if ((angle > 337.5 || angle < 22.5) ||
                  (angle > 157.5 && angle < 202.5)) {
                self.controlX = pt[0];
                self.controlY = self.targetY;
              } else if ((angle > 67.5 || angle < 112.5) ||
                  (angle > 247.5 && angle < 292.5)) {
                self.controlX = self.targetY;
                self.controlY = pt[1];
              } else if (angle > 22.5 && angle < 67.5) {
                self.controlX = self.targetX - (pt[1] - self.targetY);
                self.controlY = pt[1];
              }
            } else {
              self.controlX = pt[0];
              self.controlY = pt[1];
            }


          }
        })
        .on("end", function () {
          console.log('drag.end', this.id)
          self.onDragEnd();
        })

    d3.select("#dragdrop").selectAll("circle").call(drag);
    d3.select("#dragdrop").selectAll("rect").call(drag);

    d3.select("#control").on("contextmenu", function (evt) {
      console.log(evt, evt.shiftKey)
      evt.preventDefault();
      if (evt.shiftKey) {
        self.hasControl = false;
        self.onDragEnd();
      }
    })

    d3.select("#mainLine").on("dblclick", function (evt) {
      let pt = d3.pointer(evt, d3.select(this).node());
      self.controlX = pt[0];
      self.controlY = pt[1];
      self.hasControl = true;
    }).on("mouseover", function (evt) {
      if (!self.hasControl) {
        d3.select(this).style("cursor", "col-resize");
      }
    }).on("mouseout", function (evt) {
      d3.select(this).style("cursor", "default");
    });


    d3.selectAll("#source,#target,#control")
        .on("mouseover", function (evt) {
          d3.select(this).style("cursor", (this.id == "control" && evt.shiftKey) ? "nesw-resize" : "move");
        })
        .on("mouseout", function (evt) {
          d3.select(this).style("cursor", "default");
        })
        .on("dblclick", function (evt) {
          console.log("dblclick", this.id)
          if (self.nextAngle == 45) {
            self.hasControl = true;
          } else if (self.nextAngle == 90) {
            self.hasControl = true;
            self.controlX = self.sourceX;
            self.controlY = self.targetY;
            self.nextAngle = 90;
          }
        });
  },

  methods: {
    getAngle: function (x1, y1, x2, y2) {
      let x = x1 - x2,
          y = y1 - y2;
      if (!x && !y) {
        return 0;
      }
      return 360 - (180 + Math.atan2(-y, -x) * 180 / Math.PI + 360) % 360;
    },

    onDragEnd: function () {
      let points = [[this.targetX, this.targetY]];

      if (this.sourceX != this.target || this.sourceY != this.targetY) {
        points.push([this.sourceX, this.sourceY]);
      }

      if (this.hasControl) {
        points.push([this.controlX, this.controlY]);
      }

      this.$emit("dragged", points);
    }
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
      sourceX: this.initPoints[1] ? this.initPoints[1][0] : (this.initPoints[0] ? this.initPoints[0][0] : 100),
      sourceY: this.initPoints[1] ? this.initPoints[1][1] : (this.initPoints[0] ? this.initPoints[0][1] : 100),
      controlX: this.initPoints[2] ? this.initPoints[2][0] : 100,
      controlY: this.initPoints[2] ? this.initPoints[2][1] : 100,
      controlWidth: 10,
      hasControl: this.initPoints[2],
      nextAngle: 90,
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
