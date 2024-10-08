use std::{collections::HashMap, sync::LazyLock};

pub static GEM_INFO: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(10,""),
		(11,""),
		(12,""),
		(13,""),
		(14,""),
		(15,""),
		(16,""),
		(17,""),
		(18,""),
		(19,""),
		(20,""),
		(21,""),
		(22,""),
		(23,""),
		(24,""),
		(25,""),
		(26,""),
		(27,""),
		(28,""),
		(29,""),
		(30,""),
		(80,""),
		(81,""),
		(82,""),
		(83,""),
		(84,""),
		(85,""),
		(86,""),
		(87,""),
		(88,""),
		(89,""),
		(90,""),
		(91,""),
		(92,""),
		(93,""),
		(94,""),
		(95,""),
		(96,""),
		(97,""),
		(98,""),
		(99,""),
		(100,""),
		(101,""),
		(102,""),
		(103,""),
		(104,""),
		(105,""),
		(106,""),
		(107,""),
		(108,""),
		(109,""),
		(110,""),
		(111,""),
		(112,""),
		(113,""),
		(114,""),
		(115,""),
		(116,""),
		(117,""),
		(118,""),
		(119,""),
		(120,""),
		(121,""),
		(122,""),
		(123,""),
		(124,""),
		(125,""),
		(126,""),
		(127,""),
		(128,""),
		(129,""),
		(130,""),
		(131,""),
		(132,""),
		(133,""),
		(134,""),
		(135,""),
		(136,""),
		(137,""),
		(138,""),
		(139,""),
		(140,""),
		(141,""),
		(142,""),
		(143,""),
		(144,""),
		(145,""),
		(146,""),
		(147,""),
		(148,""),
		(149,""),
		(150,""),
		(151,""),
		(152,""),
		(153,""),
		(154,""),
		(155,""),
		(156,""),
		(157,""),
		(158,""),
		(159,""),
		(160,""),
		(161,""),
		(162,""),
		(163,""),
		(164,""),
		(165,""),
		(166,""),
		(167,""),
		(168,""),
		(169,""),
		(170,""),
		(171,""),
		(172,""),
		(173,""),
		(174,""),
		(175,""),
		(176,""),
		(177,""),
		(178,""),
		(179,""),
		(180,""),
		(181,""),
		(182,""),
		(183,""),
		(184,""),
		(185,""),
		(186,""),
		(187,""),
		(188,""),
		(189,""),
		(190,""),
		(191,""),
		(192,""),
		(193,""),
		(194,""),
		(1000,""),
		(1001,""),
		(1002,""),
		(1003,""),
		(1004,""),
		(1005,""),
		(1006,""),
		(1007,""),
		(1008,""),
		(1009,""),
		(1010,""),
		(1011,""),
		(1012,""),
		(1013,""),
		(1014,""),
		(1015,""),
		(1016,""),
		(1017,""),
		(10000,"Grants affinities and skills to an armament"),
		(10100,"Grants affinities and skills to an armament"),
		(10200,"Grants affinities and skills to an armament"),
		(10300,"Grants affinities and skills to an armament"),
		(10500,"Grants affinities and skills to an armament"),
		(10600,"Grants affinities and skills to an armament"),
		(10700,"Grants affinities and skills to an armament"),
		(10800,"Grants affinities and skills to an armament"),
		(10900,"Grants affinities and skills to an armament"),
		(11000,"Grants affinities and skills to an armament"),
		(11100,"Grants affinities and skills to an armament"),
		(11200,"Grants affinities and skills to an armament"),
		(11300,"Grants affinities and skills to an armament"),
		(11400,"Grants affinities and skills to an armament"),
		(11500,"Grants affinities and skills to an armament"),
		(11600,"Grants affinities and skills to an armament"),
		(11700,"Grants affinities and Skills to an armament"),
		(11800,"Grants affinities and skills to an armament"),
		(11900,"Grants affinities and skills to an armament"),
		(12000,"Grants affinities and skills to an armament"),
		(12100,"Assign attack type and skill to weapon"),
		(12200,"Grants affinities and skills to an armament"),
		(12300,"Grants affinities and skills to an armament"),
		(12400,"Grants affinities and skills to an armament"),
		(12500,"Grants affinities and Skills to an armament"),
		(20000,"Grants affinities and skills to an armament"),
		(20100,"Grants affinities and skills to an armament"),
		(20200,"Grants affinities and skills to an armament"),
		(20300,"Grants affinities and skills to an armament"),
		(20400,"Grants affinities and skills to an armament"),
		(20500,"Grants affinities and skills to an armament"),
		(20700,"Grants affinities and skills to an armament"),
		(20800,"Grants affinities and skills to an armament"),
		(20900,"Grants affinities and skills to an armament"),
		(21000,"Grants affinities and skills to an armament"),
		(21200,"Grants affinities and skills to an armament"),
		(21300,"Grants affinities and skills to an armament"),
		(21400,"Grants affinities and skills to an armament"),
		(21600,"Grants affinities and skills to an armament"),
		(21700,"Grants affinities and skills to an armament"),
		(21800,"Grants affinities and skills to an armament"),
		(21900,"Grants affinities and skills to an armament"),
		(22000,"Grants affinities and skills to an armament"),
		(22100,"Grants affinities and skills to an armament"),
		(22200,"Grants affinities and skills to an armament"),
		(22300,"Grants affinities and Skills to an armament"),
		(22400,"Grants affinities and skills to an armament"),
		(22500,"Grants affinities and skills to an armament"),
		(22600,"Grants affinities and skills to an armament"),
		(22700,"Grants affinities and skills to an armament"),
		(22800,"Grants affinities and skills to an armament"),
		(30000,"Grants affinities and skills to an armament"),
		(30100,"Grants affinities and skills to an armament"),
		(30200,"Grants affinities and skills to an armament"),
		(30300,"Grants affinities and Skills to an armament"),
		(30400,"Grants affinities and Skills to an armament"),
		(30500,"Grants affinities and skills to an armament"),
		(30600,"Grants affinities and skills to an armament"),
		(30700,"Grants affinities and skills to an armament"),
		(30800,"Grants affinities and skills to an armament"),
		(30900,"Grants affinities and skills to an armament"),
		(31000,"Grants affinities and skills to an armament"),
		(40000,"Grants affinities and skills to an armament"),
		(40100,"Grants affinities and skills to an armament"),
		(40200,"Grants affinities and skills to an armament"),
		(40400,"Grants affinities and skills to an armament"),
		(40500,"Grants affinities and skills to an armament"),
		(40600,"Grants affinities and skills to an armament"),
		(40700,"Assign attack type and skill to weapon"),
		(50100,"Grants affinities and skills to an armament"),
		(50200,"Grants affinities and skills to an armament"),
		(50300,"Grants affinities and skills to an armament"),
		(50400,"Grants affinities and skills to an armament"),
		(50500,"Grants affinities and skills to an armament"),
		(50600,"Grants affinities and skills to an armament"),
		(50700,"Grants affinities and skills to an armament"),
		(50800,"Grants affinities and skills to an armament"),
		(50900,"Grants affinities and skills to an armament"),
		(60000,"Grants affinities and skills to an armament"),
		(60100,"Grants affinities and skills to an armament"),
		(60200,"Grants affinities and skills to an armament"),
		(60300,"Grants affinities and skills to an armament"),
		(60400,"Grants affinities and skills to an armament"),
		(60500,"Grants affinities and skills to an armament"),
		(60600,"Grants affinities and skills to an armament"),
		(60700,"Grants affinities and skills to an armament"),
		(65000,"Grants affinities and skills to an armament"),
		(65100,"Grants affinities and skills to an armament"),
		(65200,"Grants affinities and skills to an armament"),
		(65300,"Grants affinities and skills to an armament"),
		(65400,"Grants affinities and skills to an armament"),
		(70000,"Grants affinities and skills to an armament"),
		(70100,"Grants affinities and skills to an armament"),
		(70200,"Grants affinities and skills to an armament"),
		(80000,"Grants affinities and skills to an armament"),
		(80100,"Grants affinities and skills to an armament"),
		(80200,"Grants affinities and skills to an armament"),
		(85000,"Grants affinities and skills to an armament"),
])});
