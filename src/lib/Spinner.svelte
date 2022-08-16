<div id="loading"></div>

<style>
#loading {
  display: inline-block;
  width: 1em;
  height: 1em;
  border: .1em solid rgba(255,255,255,.3);
  margin-left: auto;
  margin-right: auto;
  border-radius: 50%;
  border-top-color: #3F3;
  animation: spin 1s ease-in-out infinite;
  -webkit-animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to { -webkit-transform: rotate(360deg); }
}
@-webkit-keyframes spin {
  to { -webkit-transform: rotate(360deg); }
}
</style>