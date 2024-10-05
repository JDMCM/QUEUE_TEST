@main def eventsToCsv(file: String): Unit = {
  val eventLine = """Event: (.*)""".r
  val source = io.Source.fromFile(file)
  val csv = new java.io.PrintWriter(file + ".csv")
  csv.println("p1,p2,time,p1x,p1y,p1z,p1vx,p1vy,p1vz,p1r,p2x,p2y,p2z,p2vx,p2vy,p2vz,p2r")
  for (case eventLine(nums) <- source.getLines()) {
    csv.println(nums.split(" +").mkString(","))
  }
  csv.close()
  source.close()
}