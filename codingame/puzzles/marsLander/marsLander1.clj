(ns Player
  (:gen-class))

; Auto-generated code below aims at helping you parse
; the standard input according to the problem statement.

(defn -main [& args]
  (let [N (read)]
    ; N: the number of points used to draw the surface of Mars.
    (loop [i N]
      (when (> i 0)
        (let [LAND_X (read) LAND_Y (read)]
          ; LAND_X: X coordinate of a surface point. (0 to 6999)
          ; LAND_Y: Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        (recur (dec i)))))
    (while true
      (let [X (read) Y (read) HS (read) VS (read) F (read) R (read) P (read)]
        ; HS: the horizontal speed (in m/s), can be negative.
        ; VS: the vertical speed (in m/s), can be negative.
        ; F: the quantity of remaining fuel in liters.
        ; R: the rotation angle in degrees (-90 to 90).
        ; P: the thrust power (0 to 4).
        
        ; (binding [*out* *err*]
        ;   (println "Debug messages..."))
        
        ; R P. R is the desired rotation angle. P is the desired thrust power.
        (println (if (> (float (/ Y (+ 50 (Math/abs VS)))) (float (* (- (Math/abs VS) 39) 4))) "0 0" "0 4"))))))
