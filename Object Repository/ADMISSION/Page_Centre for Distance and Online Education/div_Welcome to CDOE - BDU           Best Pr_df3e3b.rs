<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Welcome to CDOE - BDU           Best Pr_df3e3b</name>
   <tag></tag>
   <elementGuidId>5b0dd7b8-7944-4f51-90c3-352f252f812d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.col-md-7.col-sm-6.priorty.wow.fadeInLeft</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//section[@id='about']/div/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Welcome to CDOE - BDU Best Practices CIQA Report - First Meeting CIQA - Action T&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>42b60384-21ef-4a68-a58c-8eaa47d0c499</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>col-md-7 col-sm-6 priorty wow fadeInLeft</value>
      <webElementGuid>a1f1877d-9e5e-41c3-8603-278bd3db2628</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>



        Welcome to CDOE - BDU  

         



Best Practices
CIQA Report - First Meeting
CIQA - Action Taken Report
Conduct of Placement Drives


Online Programs -  4 Quadrant approach







&lt;!--
/* 
   Rotating Fade Marquee
   Version 1.0
   November 14, 2011

   Will Bontrager Software, LLC
   http://www.willmaster.com/
   Copyright 2011 Will Bontrager Software, LLC

   This software is provided &quot;AS IS,&quot; without 
   any warranty of any kind, without even any 
   implied warranty such as merchantability 
   or fitness for a particular purpose.
   Will Bontrager Software, LLC grants 
   you a royalty free license to use this 
   software provided this notice appears on 
   all copies. 
*/

// Leave the next line as is (customization is further below):
var RM_content = new Array();

////////////////////////////////////////////////////////////
//
// Customization section.
//
// Three places to customize.
//
// Place One:
// Specify the content for each message, one per line, between 
//    quotation marks and parenthesis. Example (with ______ 
//    representing the content):
//       RM_content.push(&quot;______&quot;);
//    Content may contain HTML code (like links and CSS), even 
//    image tags.
// If the content itself contains quotation marks, precede 
//    each quotation mark with a backslash character. 
//    Example: He said, \&quot;five!\&quot;

RM_content.push(&quot;Learner Support Centres across the Territory&quot;);
RM_content.push(&quot;Tuition Fee payment in Easy Installments &quot;);
RM_content.push(&quot;Easy Online Payment Option&quot;);
RM_content.push(&quot;Innovative Undergraduate and Post Graduate Programmes&quot;);
RM_content.push(&quot;Quick response to grievances&quot;);
RM_content.push(&quot;Open Source Online Study Materials&quot;);
RM_content.push(&quot;Online question bank (Previous semester question papers)&quot;);
RM_content.push(&quot;Easy Transfer from Regular College to Distance Education&quot;);
RM_content.push(&quot;State-of-the-art Infrastructure facilities&quot;);
RM_content.push(&quot;Scholarships for eligible students&quot;);



//RM_content.push(&quot;You'll find lots of free stuff on the &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>programmer's website&lt;/u>&lt;/a>.&quot;);
//RM_content.push(&quot;Also commercial software and WordPress plugins.&quot;);
//RM_content.push(&quot;The programmer does custom work for special requirements.&quot;);
//RM_content.push(&quot;Head over &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>to the website &lt;b>now&lt;/b>&lt;/u>&lt;/a> and get your goodies.&quot;);


// Place Two:
// Specify the number of seconds to pause between displaying 
//    one marquee and the next. A decimal number is acceptable.

var RM_PauseBetweenEach = 2.0;


// Place Three:
// Transitions are from 100% opacity to 0%, then from 0% to 100%.
//
// Two transition preferences can be specified. The number of 
//    transition steps per fade and how fast the steps shall 
//    occur.
// For the steps, the larger the number, the smoother and slower 
//    the transition.
// For the speed, the lower the number the faster the transition.

var RM_TransitionSteps = 25; // Number of steps per fade.
var RM_TransitionSpeed = 50; // How fast the steps shall occur.


// End of customization section.
////////////////////////////////////////////////////////////
RM_TransitionSteps = parseInt( (100 / RM_TransitionSteps) + .5 );
var RMlastPointer = RM_content.length - 1;
var RMopacity = 100;
var RMpointer = 0;
var RMfader;
var RMdiv;
var RMie;

function RM_StartRotateMarquee() {
RMdiv = document.getElementById(&quot;RM_FadeInOutContentDiv&quot;);
RMie = (RMdiv.filters) ? true : false;
RMdiv.innerHTML = RM_content[RMpointer];
setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
}

function RM_NewOpacity() {
if( RMie ) { RMdiv.filters.alpha.opacity = RMopacity; }
else { RMdiv.style.opacity = RMopacity/100; }
}

function RM_FadeOut() {
RMopacity -= RM_TransitionSteps;
if( RMopacity &lt; 1 ) { RMopacity = 0; }
RM_NewOpacity(RMopacity);
if( RMopacity &lt; 1 ) { 
   clearInterval(RMfader);
   RM_SwitchContent();
   }
}

function RM_FadeIn() {
RMopacity += RM_TransitionSteps;
if( RMopacity > 99 ) { RMopacity = 100; }
RM_NewOpacity(RMopacity);
if( RMopacity > 99 ) {
   clearInterval(RMfader);
   setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
   }
}

function RM_NextContent() {
RMfader = setInterval( &quot;RM_FadeOut()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_SwitchContent() {
RMpointer++;
if( RMpointer > RMlastPointer ) { RMpointer = 0; }
RMdiv.innerHTML = RM_content[RMpointer];
RMfader = setInterval( &quot;RM_FadeIn()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_AddOnloadEvent(f) {
if(typeof window.onload != 'function') { window.onload = f; }
else {
   var cache = window.onload;
   window.onload = function() {
      if(cache) { cache(); }
      f();
      };
   }
}
RM_AddOnloadEvent(RM_StartRotateMarquee);
//-->
 

Tuition Fee payment in Easy Installments 


        The Centre for Distance and Online Education (CDOE) of Bharathidasan University (BDU) was established in the year 1992 to serve the students who could not enter the regular colleges for higher education. As the educational programmes offered and the degrees awarded through distance mode are on par with the regular mode, qualitatively there is a demand for the programmes offered by the CDE of Bharathidasan University.
        

UGC – Public Notice – Precautions to be taken by the students before enrolling in Programmes offered under Open &amp; Distance Learning (ODL) and/or Online Learning Mode






 





	                
	  	

        

           
            
            
            About CDE
            
            

            
            
            Learner Support Centres
            
            


            
            
            Question Bank
            
             


            
            
            Class Timetable
            
            
          

          
            
            
            Programmes Offered
            
            

            
            
            Admissions (2024 Calendar Year)
            
            

            
            
            Question Paper Directory
            
            


            
            
            Uniform Span Period
            
            



          
        

      </value>
      <webElementGuid>a73b6543-fbbd-4ed2-9c1b-e9d055d4ecae</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;about&quot;)/div[@class=&quot;container&quot;]/div[@class=&quot;row&quot;]/div[@class=&quot;col-md-7 col-sm-6 priorty wow fadeInLeft&quot;]</value>
      <webElementGuid>b0414890-a983-4d49-87fa-e361638154fd</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//section[@id='about']/div/div/div</value>
      <webElementGuid>2256879e-674e-41a5-b36e-02f901350eef</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Search'])[1]/following::div[5]</value>
      <webElementGuid>3ee98e4d-e9c9-4ef8-a5f1-86800ef859c4</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='×'])[1]/following::div[5]</value>
      <webElementGuid>b3997ba3-b292-4d19-a4df-cb3202889fca</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section[2]/div/div/div</value>
      <webElementGuid>01543a82-8580-4b4b-b385-9cda22f10d6e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;



        Welcome to CDOE - BDU  

         



Best Practices
CIQA Report - First Meeting
CIQA - Action Taken Report
Conduct of Placement Drives


Online Programs -  4 Quadrant approach







&lt;!--
/* 
   Rotating Fade Marquee
   Version 1.0
   November 14, 2011

   Will Bontrager Software, LLC
   http://www.willmaster.com/
   Copyright 2011 Will Bontrager Software, LLC

   This software is provided &quot;AS IS,&quot; without 
   any warranty of any kind, without even any 
   implied warranty such as merchantability 
   or fitness for a particular purpose.
   Will Bontrager Software, LLC grants 
   you a royalty free license to use this 
   software provided this notice appears on 
   all copies. 
*/

// Leave the next line as is (customization is further below):
var RM_content = new Array();

////////////////////////////////////////////////////////////
//
// Customization section.
//
// Three places to customize.
//
// Place One:
// Specify the content for each message, one per line, between 
//    quotation marks and parenthesis. Example (with ______ 
//    representing the content):
//       RM_content.push(&quot;______&quot;);
//    Content may contain HTML code (like links and CSS), even 
//    image tags.
// If the content itself contains quotation marks, precede 
//    each quotation mark with a backslash character. 
//    Example: He said, \&quot;five!\&quot;

RM_content.push(&quot;Learner Support Centres across the Territory&quot;);
RM_content.push(&quot;Tuition Fee payment in Easy Installments &quot;);
RM_content.push(&quot;Easy Online Payment Option&quot;);
RM_content.push(&quot;Innovative Undergraduate and Post Graduate Programmes&quot;);
RM_content.push(&quot;Quick response to grievances&quot;);
RM_content.push(&quot;Open Source Online Study Materials&quot;);
RM_content.push(&quot;Online question bank (Previous semester question papers)&quot;);
RM_content.push(&quot;Easy Transfer from Regular College to Distance Education&quot;);
RM_content.push(&quot;State-of-the-art Infrastructure facilities&quot;);
RM_content.push(&quot;Scholarships for eligible students&quot;);



//RM_content.push(&quot;You&quot; , &quot;'&quot; , &quot;ll find lots of free stuff on the &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>programmer&quot; , &quot;'&quot; , &quot;s website&lt;/u>&lt;/a>.&quot;);
//RM_content.push(&quot;Also commercial software and WordPress plugins.&quot;);
//RM_content.push(&quot;The programmer does custom work for special requirements.&quot;);
//RM_content.push(&quot;Head over &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>to the website &lt;b>now&lt;/b>&lt;/u>&lt;/a> and get your goodies.&quot;);


// Place Two:
// Specify the number of seconds to pause between displaying 
//    one marquee and the next. A decimal number is acceptable.

var RM_PauseBetweenEach = 2.0;


// Place Three:
// Transitions are from 100% opacity to 0%, then from 0% to 100%.
//
// Two transition preferences can be specified. The number of 
//    transition steps per fade and how fast the steps shall 
//    occur.
// For the steps, the larger the number, the smoother and slower 
//    the transition.
// For the speed, the lower the number the faster the transition.

var RM_TransitionSteps = 25; // Number of steps per fade.
var RM_TransitionSpeed = 50; // How fast the steps shall occur.


// End of customization section.
////////////////////////////////////////////////////////////
RM_TransitionSteps = parseInt( (100 / RM_TransitionSteps) + .5 );
var RMlastPointer = RM_content.length - 1;
var RMopacity = 100;
var RMpointer = 0;
var RMfader;
var RMdiv;
var RMie;

function RM_StartRotateMarquee() {
RMdiv = document.getElementById(&quot;RM_FadeInOutContentDiv&quot;);
RMie = (RMdiv.filters) ? true : false;
RMdiv.innerHTML = RM_content[RMpointer];
setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
}

function RM_NewOpacity() {
if( RMie ) { RMdiv.filters.alpha.opacity = RMopacity; }
else { RMdiv.style.opacity = RMopacity/100; }
}

function RM_FadeOut() {
RMopacity -= RM_TransitionSteps;
if( RMopacity &lt; 1 ) { RMopacity = 0; }
RM_NewOpacity(RMopacity);
if( RMopacity &lt; 1 ) { 
   clearInterval(RMfader);
   RM_SwitchContent();
   }
}

function RM_FadeIn() {
RMopacity += RM_TransitionSteps;
if( RMopacity > 99 ) { RMopacity = 100; }
RM_NewOpacity(RMopacity);
if( RMopacity > 99 ) {
   clearInterval(RMfader);
   setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
   }
}

function RM_NextContent() {
RMfader = setInterval( &quot;RM_FadeOut()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_SwitchContent() {
RMpointer++;
if( RMpointer > RMlastPointer ) { RMpointer = 0; }
RMdiv.innerHTML = RM_content[RMpointer];
RMfader = setInterval( &quot;RM_FadeIn()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_AddOnloadEvent(f) {
if(typeof window.onload != &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) { window.onload = f; }
else {
   var cache = window.onload;
   window.onload = function() {
      if(cache) { cache(); }
      f();
      };
   }
}
RM_AddOnloadEvent(RM_StartRotateMarquee);
//-->
 

Tuition Fee payment in Easy Installments 


        The Centre for Distance and Online Education (CDOE) of Bharathidasan University (BDU) was established in the year 1992 to serve the students who could not enter the regular colleges for higher education. As the educational programmes offered and the degrees awarded through distance mode are on par with the regular mode, qualitatively there is a demand for the programmes offered by the CDE of Bharathidasan University.
        

UGC – Public Notice – Precautions to be taken by the students before enrolling in Programmes offered under Open &amp; Distance Learning (ODL) and/or Online Learning Mode






 





	                
	  	

        

           
            
            
            About CDE
            
            

            
            
            Learner Support Centres
            
            


            
            
            Question Bank
            
             


            
            
            Class Timetable
            
            
          

          
            
            
            Programmes Offered
            
            

            
            
            Admissions (2024 Calendar Year)
            
            

            
            
            Question Paper Directory
            
            


            
            
            Uniform Span Period
            
            



          
        

      &quot;) or . = concat(&quot;



        Welcome to CDOE - BDU  

         



Best Practices
CIQA Report - First Meeting
CIQA - Action Taken Report
Conduct of Placement Drives


Online Programs -  4 Quadrant approach







&lt;!--
/* 
   Rotating Fade Marquee
   Version 1.0
   November 14, 2011

   Will Bontrager Software, LLC
   http://www.willmaster.com/
   Copyright 2011 Will Bontrager Software, LLC

   This software is provided &quot;AS IS,&quot; without 
   any warranty of any kind, without even any 
   implied warranty such as merchantability 
   or fitness for a particular purpose.
   Will Bontrager Software, LLC grants 
   you a royalty free license to use this 
   software provided this notice appears on 
   all copies. 
*/

// Leave the next line as is (customization is further below):
var RM_content = new Array();

////////////////////////////////////////////////////////////
//
// Customization section.
//
// Three places to customize.
//
// Place One:
// Specify the content for each message, one per line, between 
//    quotation marks and parenthesis. Example (with ______ 
//    representing the content):
//       RM_content.push(&quot;______&quot;);
//    Content may contain HTML code (like links and CSS), even 
//    image tags.
// If the content itself contains quotation marks, precede 
//    each quotation mark with a backslash character. 
//    Example: He said, \&quot;five!\&quot;

RM_content.push(&quot;Learner Support Centres across the Territory&quot;);
RM_content.push(&quot;Tuition Fee payment in Easy Installments &quot;);
RM_content.push(&quot;Easy Online Payment Option&quot;);
RM_content.push(&quot;Innovative Undergraduate and Post Graduate Programmes&quot;);
RM_content.push(&quot;Quick response to grievances&quot;);
RM_content.push(&quot;Open Source Online Study Materials&quot;);
RM_content.push(&quot;Online question bank (Previous semester question papers)&quot;);
RM_content.push(&quot;Easy Transfer from Regular College to Distance Education&quot;);
RM_content.push(&quot;State-of-the-art Infrastructure facilities&quot;);
RM_content.push(&quot;Scholarships for eligible students&quot;);



//RM_content.push(&quot;You&quot; , &quot;'&quot; , &quot;ll find lots of free stuff on the &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>programmer&quot; , &quot;'&quot; , &quot;s website&lt;/u>&lt;/a>.&quot;);
//RM_content.push(&quot;Also commercial software and WordPress plugins.&quot;);
//RM_content.push(&quot;The programmer does custom work for special requirements.&quot;);
//RM_content.push(&quot;Head over &lt;a href=\&quot;http://www.willmaster.com/\&quot; target=\&quot;_blank\&quot;>&lt;u>to the website &lt;b>now&lt;/b>&lt;/u>&lt;/a> and get your goodies.&quot;);


// Place Two:
// Specify the number of seconds to pause between displaying 
//    one marquee and the next. A decimal number is acceptable.

var RM_PauseBetweenEach = 2.0;


// Place Three:
// Transitions are from 100% opacity to 0%, then from 0% to 100%.
//
// Two transition preferences can be specified. The number of 
//    transition steps per fade and how fast the steps shall 
//    occur.
// For the steps, the larger the number, the smoother and slower 
//    the transition.
// For the speed, the lower the number the faster the transition.

var RM_TransitionSteps = 25; // Number of steps per fade.
var RM_TransitionSpeed = 50; // How fast the steps shall occur.


// End of customization section.
////////////////////////////////////////////////////////////
RM_TransitionSteps = parseInt( (100 / RM_TransitionSteps) + .5 );
var RMlastPointer = RM_content.length - 1;
var RMopacity = 100;
var RMpointer = 0;
var RMfader;
var RMdiv;
var RMie;

function RM_StartRotateMarquee() {
RMdiv = document.getElementById(&quot;RM_FadeInOutContentDiv&quot;);
RMie = (RMdiv.filters) ? true : false;
RMdiv.innerHTML = RM_content[RMpointer];
setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
}

function RM_NewOpacity() {
if( RMie ) { RMdiv.filters.alpha.opacity = RMopacity; }
else { RMdiv.style.opacity = RMopacity/100; }
}

function RM_FadeOut() {
RMopacity -= RM_TransitionSteps;
if( RMopacity &lt; 1 ) { RMopacity = 0; }
RM_NewOpacity(RMopacity);
if( RMopacity &lt; 1 ) { 
   clearInterval(RMfader);
   RM_SwitchContent();
   }
}

function RM_FadeIn() {
RMopacity += RM_TransitionSteps;
if( RMopacity > 99 ) { RMopacity = 100; }
RM_NewOpacity(RMopacity);
if( RMopacity > 99 ) {
   clearInterval(RMfader);
   setTimeout( &quot;RM_NextContent()&quot;, parseInt(RM_PauseBetweenEach*1000) );
   }
}

function RM_NextContent() {
RMfader = setInterval( &quot;RM_FadeOut()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_SwitchContent() {
RMpointer++;
if( RMpointer > RMlastPointer ) { RMpointer = 0; }
RMdiv.innerHTML = RM_content[RMpointer];
RMfader = setInterval( &quot;RM_FadeIn()&quot;, parseInt(RM_TransitionSpeed) );
}

function RM_AddOnloadEvent(f) {
if(typeof window.onload != &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) { window.onload = f; }
else {
   var cache = window.onload;
   window.onload = function() {
      if(cache) { cache(); }
      f();
      };
   }
}
RM_AddOnloadEvent(RM_StartRotateMarquee);
//-->
 

Tuition Fee payment in Easy Installments 


        The Centre for Distance and Online Education (CDOE) of Bharathidasan University (BDU) was established in the year 1992 to serve the students who could not enter the regular colleges for higher education. As the educational programmes offered and the degrees awarded through distance mode are on par with the regular mode, qualitatively there is a demand for the programmes offered by the CDE of Bharathidasan University.
        

UGC – Public Notice – Precautions to be taken by the students before enrolling in Programmes offered under Open &amp; Distance Learning (ODL) and/or Online Learning Mode






 





	                
	  	

        

           
            
            
            About CDE
            
            

            
            
            Learner Support Centres
            
            


            
            
            Question Bank
            
             


            
            
            Class Timetable
            
            
          

          
            
            
            Programmes Offered
            
            

            
            
            Admissions (2024 Calendar Year)
            
            

            
            
            Question Paper Directory
            
            


            
            
            Uniform Span Period
            
            



          
        

      &quot;))]</value>
      <webElementGuid>a9b9c879-c636-46e3-aa86-65014d91aa3a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
