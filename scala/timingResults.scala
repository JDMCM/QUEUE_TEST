import scala.collection.immutable.ArraySeq

val counts = ArraySeq(10000, 30000, 100000, 300000, 1000000)

def calc(nums: ArraySeq[Double]): (Double, Double) = {
  val avg = nums.sum / nums.length
  val std = nums.map(_ - avg).map(x => x*x).sum
  (avg, math.sqrt(std / nums.length))
}

def printForSwiftVis(results: ArraySeq[ArraySeq[(Double, Double)]]): Unit = {
  for ((batch, index) <- results.zipWithIndex) {
    for (((v, std), cnt) <- batch.zip(counts)) {
      println(s"$index $cnt $v $std") 
    }
  }
}

@main def timingResults(): Unit = {
  val seqBH = ArraySeq(
    ArraySeq(13.47e-3, 11.66e-3, 10.68e-3, 10.52e-3, 13.64e-3, 10.64e-3, 10.86e-3),
    ArraySeq(90.14e-3, 91.19e-3, 90.69e-3, 85.12e-3, 90.62e-3, 91.12e-3, 91.07e-3),
    ArraySeq(0.91171, 0.92009, 0.86314, 0.86873, 0.86219, 0.90409, 0.88779),
    ArraySeq(6.58, 6.65, 6.99, 6.73, 6.99, 6.96, 6.83),
    ArraySeq(152.72, 148.07, 154.23, 154.14, 155.98, 155.63, 153.99),
  )
  val seqBQ = ArraySeq(
    ArraySeq(13.46e-3, 11.06e-3, 10.39e-3, 10.21e-3, 14.01e-3, 11.15e-3, 10.97e-3),
    ArraySeq(63.39e-3, 65.38e-3, 62.67e-3, 59.04e-3, 63.31e-3, 64.56e-3, 63.95e-3),
    ArraySeq(0.47541, 0.75445, 0.44819, 0.44698, 0.45274, 0.47397, 0.45165),
    ArraySeq(4.68, 4.72, 5.1, 4.77, 5.07, 5.08, 4.94),
    ArraySeq(146.98, 139.54, 145.24, 146.77, 146.95, 146.64, 149.8),
  )
  val parBH = ArraySeq(
    ArraySeq(0.82799, 0.80446, 0.85318, 0.82381, 0.79944, 0.84226, 0.81926),
    ArraySeq(1.78, 1.74, 1.78, 1.73, 1.73, 1.74, 1.79),
    ArraySeq(3.44, 3.61, 3.43, 3.52, 3.51, 3.63, 3.52),
    ArraySeq(6.98, 7.16, 6.88, 7.11, 7.16, 6.90, 7.02),
    ArraySeq(33.52, 33.59, 34.23, 33.56, 33.45, 33.95, 33.56),
  )
  val parBQ = ArraySeq(
    ArraySeq(0.17237, 0.16929, 0.1739, 0.18605, 0.19714, 0.17335, 0.16927),
    ArraySeq(0.26223, 0.26035, 0.26639, 0.24887, 0.26967, 0.28687, 0.26245),
    ArraySeq(0.76756, 0.56757, 0.51473, 0.53137, 0.5556, 0.65587, 0.63866),
    ArraySeq(0.99024, 0.93515, 0.92963, 1.06, 0.94307, 0.93921, 0.93586),
    ArraySeq(9.34, 9.72, 9.11, 9.31, 9.98, 9.49, 9.15),
  )

  println(s"seqBH = ${seqBH.map(calc)}")
  println(s"seqBQ = ${seqBQ.map(calc)}")
  println(s"parBH = ${parBH.map(calc)}")
  println(s"parBQ = ${parBQ.map(calc)}")

  printForSwiftVis(ArraySeq(seqBH.map(calc), seqBQ.map(calc), parBH.map(calc), parBQ.map(calc)))
}
