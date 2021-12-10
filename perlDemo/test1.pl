#!perl

use strict;
use Getopt::Long;
use MaterialsScript qw(:all);

my $doc = $Documents{"13X1.xsd"};
my $newStudyTable = Documents->New("123.std");
my $calcSheet = $newStudyTable->ActiveSheet;
$newStudyTable->ActiveSheet->ColumnHeading(0) = "radius";
$newStudyTable->ActiveSheet->ColumnHeading(1) = "number";

my $atom1 = $doc->UnitCell->Sets("NA3")->Atoms;

my $A=11.5030;

my $B=23.3694;

my $C=21.5075;

my $count=1;
for (my $i = 1; $i<=30; $i++)
{
    my $num = 0;
    print($1);
    foreach my $atom11 (@$atom1)
    {
        my $X1=$atom11->X;
        my $Y1=$atom11->Y;
        my $Z1=$atom11->Z;

        if (($X1-$A)**2+($Y1-$B)**2+($Z1-$C)**2<=25)

        {$num = $num + 1;}}
    $calcSheet->Cell($count-1, 0) =$i;
    $calcSheet->Cell($count-1, 1) =$num;
    $count=$count+1;
}