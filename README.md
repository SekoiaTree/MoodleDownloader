# MoodleDownloader
A downloader for the EPFL moodle. Will break really easily if the page changes much.

To call, pass as a first argument the cookie "MoodleSession"; this will allow the fetcher to get access to the pages. This key changes relatively regularly.

The second part is all of the links you want to get; copy the link directly from the moodle page.

Some useful scripts:
```js
{
let children = document.getElementsByClassName("fp-filename-icon");
var text = "";
for (let i = 0; i < children.length; i++) {
  if (children[i].tagName === "SPAN") {
      text += children[i].firstChild.href + " ";
  }
}
text
}
```
The above will return all of the links in a moodle folder. Run it in a browser console.

```js
{
let children = document.getElementsByClassName("activity resource modtype_resource");
var text = "";
for (let i = 0; i < children.length; i++) {
  text += children[i].getElementsByTagName('a')[0].href+" ";
}
text
}
```
The above will return all file links on a regular moodle page. Run it in a browser console.
