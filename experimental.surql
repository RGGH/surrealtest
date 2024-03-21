REMOVE table driver;
REMOVE table team;
REMOVE table scored;

DEFINE TABLE team SCHEMALESS PERMISSIONS NONE;
CREATE team SET name="Alpine";
CREATE team SET name="Aston Martin";
CREATE team SET name="Ferrari";
CREATE team SET name="Haas";
CREATE team SET name="Sauber";
CREATE team SET name="McLaren";
CREATE team SET name="Mercedes";
CREATE team SET name="RB";
CREATE team SET name="Red Bull";
CREATE team SET name="Williams";

DEFINE TABLE drivers SCHEMALESS PERMISSIONS NONE;
CREATE driver SET name="Pierre Gasly",team.name=Alpine;
CREATE driver SET name="Esteban Ocon",team.name=Alpine;


Let $dn = SELECT VALUE id from driver where name="Pierre Gasly";
Let $tn = SELECT VALUE id FROM team where name="Alpine"; 
RELATE $dn->scored->$tn
CONTENT {
    2024: {
		Dubai: {
			finished: 18
		    },
        Saudi: {
			finished: 18
		    },
        }
	};

Let $dn = SELECT VALUE id from driver where name="Esteban Ocon";

RELATE $dn->scored->$tn
CONTENT {
    2024: {
		Dubai: {
			finished: 19
		    },
        Saudi: {
			finished: 17
		    },
        }
	};


Let $tn = SELECT VALUE id FROM team where name="Alpine"; 

-- race result scored by Alpine Drivers in Dubai - 2024
SELECT in.name as driver,2024.Dubai.finished FROM scored;

-- race result scored by Alpine Drivers in Saudi - 2024
SELECT in.name as driver,2024.Saudi.finished FROM scored;
