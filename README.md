# Exercice Cache LRU

## Consignes:
Un cache LRU est un algo d'optimisation qui permet à un programme qui doit aller chercher de la donnée qui revient souvent
de la récupérer de façon plus rapide. On va enregistrer dans ce cache les données les plus récemment utilisées.
Ce cache a une taille limite. Disons qu'il a une taille de 3, ca voudra dire qu'il va toujours garder en cache les 3 derniers éléments
qu'il a voulu récupérer sur une API par exemple. Donc si je fais 4 call http et que je veux mettre un cache devant ces calls HTTP pour ne
pas les faire si jamais je les ai déjà récupéré il y a peu, je vais commencer avec un cache vide.

Exemple de cas d'usage d'un cache LRU:
1) je regarde dans mon cache si j'ai la donnée A ? non -> je récupère la donnée A de mon call HTTP et je la mets dans mon cache en même temps
2) je regarde dans mon cache si j'ai la donnée B ? non -> je récupère ma donnée B de mon call HTTP et je la mets dans mon cache en même temps
3) je regarde dans mon cache si j'ai la donnée C ? non -> je récupère ma donnée C de mon call HTTP et je la mets dans mon cache en même temps
4) je regarde dans mon cache si j'ai la donnée D ? non -> je récupère ma donnée D de mon call HTTP et je la mets dans mon cache en même temps mais
   je supprime A de mon cache car mon cache fait une taille de 3 ( je ne peux enregistrer que maximum 3 entrées);
5) je regarde dans mon cache si j'ai la donnée B ? OUI -> je récupère ma donnée B de mon CACHE et en la récupérant mon cache enregistre que
   je viens de l'utiliser donc la donnée B a été utilisée récemment
6) je regarde dans mon cache si j'ai la donnée A ? NON puisque mon cache à une taille de 3 donc le A a été supprimé de mon cache quand j'ai
   enregistré D dans mon cache
7) je regarde dans mon cache si j'ai la donnée E ? non -> je récupère ma donnée E de mon call HTTP et je la mets dans mon cache en même temps mais
   je supprime C (et non pas B car je l'ai récupérée récemment, cf step 5) de mon cache car mon cache fait une taille de 3 ( je ne peux enregistrer que maximum 3 entrées);
   ....

Je souhaite que vous me fassiez un cache LRU avec une struct qui est capable d'enregistrer un peu comme une hashmap des données clés valeurs
Exemple d'utilisation

### Itérations:
1) Rendre la valeur générique (donc pas forcément un truc String)
2) Utiliser un trait plutôt que des méthodes
3) Rendre générique la clé
3) Cache qui se store dans un fichier (Cache::new_persistent(3, "mon_cache.txt"))
4) Performances (https:crates.io/crates/criterion si vous voulez mesurer)

### Pré-requis:
- PAS D'UTILISATION DE LIBS EXTERNES.
- Concepts de borrow checking acquis
- Concepts de struct et implementation
- Je veux une librairie avec des sous modules, que ce soit bien architecturé
- Je veux des tests unitaires et intégrations
- Je veux de la documentation avec des exemples fournis dans la doc
