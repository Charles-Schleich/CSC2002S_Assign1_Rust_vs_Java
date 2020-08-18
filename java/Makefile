JAVAC=/usr/bin/javac
.SUFFIXES: .java .class

SRCDIR=src
BINDIR=bin

$(BINDIR)/%.class:$(SRCDIR)/%.java
	$(JAVAC) -d $(BINDIR)/ -cp $(BINDIR) $<

CLASSES=\
	SumArray.class \
	SumAll.class \
	Tuple.class \
	Sequential.class \
	ProcessParallel.class \
	Parallel.class \


CLASS_FILES=$(CLASSES:%.class=$(BINDIR)/%.class)

default: $(CLASS_FILES)

clean:
	rm $(BINDIR)/*.class
run:
	@java -cp bin SumAll 

runseq:
	@java -cp bin Sequential large_in.txt 

runpar:
	@java -cp bin Parallel  large_in.txt large_out.txt 

run_sum:
	@java -cp bin SumAll
