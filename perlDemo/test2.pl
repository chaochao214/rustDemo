#!perl

use strict;
use Getopt::Long;
use MaterialsScript qw(:all);

my $doc = $Documents{"13X.xsd"};
my $newStudyTable = Documents->New("123.std");
my $calcSheet = $newStudyTable->ActiveSheet;
$newStudyTable->ActiveSheet->ColumnHeading(0) = "radius";
$newStudyTable->ActiveSheet->ColumnHeading(1) = "number";

my $atom1 = $doc->UnitCell->Sets("NA3")->Atoms;
my $atom2 = $doc->UnitCell->Sets("NA2")->Atoms;
my $A=11.5030;

my $B=23.3694;

my $C=21.5075;

my $count=1;
for (my $i = 1; $i<=30; $i++)
{
    my $num = 0;
    foreach my $atom11 (@$atom1,$atom2)
    {
        my $X1=$atom11->$atom1->X;
        my $Y1=$atom11->$atom1->Y;
        my $Z1=$atom11->$atom1->Z;
        my $X2=$atom22->$atom2->X;
        my $Y2=$atom22->$atom2->Y;
        my $Z2=$atom22->$atom2->Z;
        if ((($X1-$A)**2+($Y1-$B)**2+($Z1-$C)**2<=25)
            &&
            (($X2-$A)**2+($Y2-$B)**2+($Z2-$C)**2<=25))
        {$num = $num + 1;}}
    $calcSheet->Cell($count-1, 0) =$i;
    $calcSheet->Cell($count-1, 1) =$num;
    $count=$count+1;}