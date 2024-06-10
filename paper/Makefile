pdf:
	pdflatex ./paper.tex
	biber ./paper
	pdflatex ./paper.tex
	pdflatex ./paper.tex

clean:
	rm -f *.log *.aux *.bbl *.blg *.out *.bcf *.pdf *.run.xml