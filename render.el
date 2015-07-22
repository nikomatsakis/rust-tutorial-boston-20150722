(save-excursion
  ;; clear existing index.md
  (switch-to-buffer "index.md")
  (end-of-buffer)
  (kill-region (point-min) (point-max))

  (insert "### Tutorial examples\n\n")

  (insert "- [Directions for installing Rust][d], should you choose to do so.\n")
  (insert "- [API docs][api] for Rust.\n")

  (let ((examples '(01
                    02
                    03)))

    ;; Generate the html files.
    (dolist (n examples)
      (switch-to-buffer (format "exercise%02d.rs" n))
      (htmlfontify-buffer)
      (switch-to-buffer (format "exercise%02d.rs.html" n))
      (beginning-of-buffer)
      (replace-string "font-size: 0pt;" "font-size: 12pt;")
      (save-buffer)
      (kill-buffer))

    ;; Insert the index.md links
    (switch-to-buffer "index.md")
    (end-of-buffer)
    (dolist (n examples)
      (insert (format "- Exercise %02d: [html][html%d], [play][play%d]\n"
                      n n n)))

    ;; Generate the html links
    (insert "\n")
    (dolist (n examples)
      (insert (format "[html%d]: exercise%02d.rs.html\n" n n)))

    ;; Generate the actual links
    (dolist (n examples)
      (switch-to-buffer (format "exercise%02d.rs" n))
      (let* ((text (buffer-string))
             (text (replace-regexp-in-string "/\\*[^/]*\\*/" "" text)))
        (switch-to-buffer "index.md")
        (end-of-buffer)
        (insert (format "[play%d]: http://play.rust-lang.org/?code=%s\n"
                        n (url-hexify-string text)))))
    )

  (switch-to-buffer "index.md")
  (end-of-buffer)
  (insert "[api]: http://doc.rust-lang.org/std/index.html\n")
  (insert "[d]: http://doc.rust-lang.org/guide.html#installing-rust\n")
  (save-buffer)
  )

