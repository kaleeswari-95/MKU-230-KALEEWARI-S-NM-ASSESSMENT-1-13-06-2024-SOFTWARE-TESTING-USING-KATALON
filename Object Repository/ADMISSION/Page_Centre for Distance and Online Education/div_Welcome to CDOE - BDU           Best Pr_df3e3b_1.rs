<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Welcome to CDOE - BDU           Best Pr_df3e3b_1</name>
   <tag></tag>
   <elementGuidId>45cb1c12-75c5-461f-924d-a45760d94d48</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#about > div.container > div.row</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//section[@id='about']/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>div >> internal:has-text=&quot;Welcome to CDOE - BDU Best Practices CIQA Report - First Meeting CIQA - Action T&quot;i >> nth=2</value>
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
      <webElementGuid>9213cefb-5a85-4cbb-ab3d-1d825426df3d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>row</value>
      <webElementGuid>94208540-1698-4c12-8006-f97f982ef70b</webElementGuid>
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
            
            



          
        

      


      
April 2024 Exam Timetable 

      
April 2024 Exam Notification 

      
Nov'2023 Exam Results 

      
Class Timetable 


      
Namelist with Register No. 

      
Exam. Application Form

      

Syllabi (UG, PG &amp; Diploma)  

      
	  
	  
    



UG Non-Semester Programmes -  Part-1 Urudu and Sanskrit - Exam Timetable
MBA and MCA Semester Programmes -  Examination Timetable - April 2024
UG and PG Non-Semester Programmes -  Examination Timetable - April 2024

MCA Program - 2024 Syllabi- Subject Code details uploaded  


UG Non-Semester Programmes April 2024 Examination Notification (Tamil &amp; English)  

MBA and MCA April 2024 Exam Notification



UG - November 2023 Exam Results



I Semester MBA Program (2024 Calender Year batch) - Class Timetable 


I Semester MBA Program (2024 Calender Year batch) - Assignment Topics 


MBA AND MCA - November 2023 Exam Results Updated

PG - November 2023 Exam Results


MBA - IV Semester Project Coordinators List


MBA - IV Semester Project - Review Meeting Dates



I Year PG Arts Programs (2023 Academic Year batch) Trichy PCP - Class Timetable



I Year PG Science Programs (2023 Academic Year batch) Trichy PCP - Class Timetable

I Year PG (2023 Academic Year batch) - Class Timetable


III Semester MBA (2023 Calendar Year batch) - Assignment Topics


Calendar year 2022 (III Year), 2023 (II Year) - Tuition Fee Circular


MBA II, III and IV SEMESTER Fee Circular

1. MBA II SEMESTER CLASS TIME TABLE (ACADEMIC YEAR 2023-2025)
2. MBA II SEMESTER (ACADEMIC YEAR-2023-2025) ASSIGNMENT TOPICS
3. MBA III YEAR CLASS TIME TABLE (CALENDAR YEAR 2023)
4. MBA III SEMESTER (CALENDAR YEAR-2023) ASSIGNMENT TOPICS
5. MBA III SEMESTER ELECTIVE (CALENDAR YEAR 2023)
6.MBA IV YEAR CLASS TIME TABLE (ACADEMIC YEAR 2022-2024)
7. MBA IV SEMESTER (ACADEMIC YEAR-2022-2024) ASSIGNMENT TOPICS


UG and PG Non Semester, MBA, MCA and B.Ed. - November 2023 Exam Timetable 


II year and III Year UG - PCP Centres Class timetable. Click to view 



 

















 












    
	  
    
                            
    View all Notifications
    

      

      </value>
      <webElementGuid>0cb06c2d-ee4c-4f49-8ba3-d0ad41ffb6c8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;about&quot;)/div[@class=&quot;container&quot;]/div[@class=&quot;row&quot;]</value>
      <webElementGuid>f47a8d3b-b358-420a-9363-bc25d33ff1da</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//section[@id='about']/div/div</value>
      <webElementGuid>6af3d105-c80a-436b-b14c-a061a9dbb51e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Search'])[1]/following::div[4]</value>
      <webElementGuid>777a90e6-c165-465b-b261-e02e938ef987</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='×'])[1]/following::div[4]</value>
      <webElementGuid>6aee1085-818c-4f52-8de1-9234fea3f5dc</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section[2]/div/div</value>
      <webElementGuid>7392808d-1de8-44e0-8a5c-f2ef4e955c46</webElementGuid>
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
            
            



          
        

      


      
April 2024 Exam Timetable 

      
April 2024 Exam Notification 

      
Nov&quot; , &quot;'&quot; , &quot;2023 Exam Results 

      
Class Timetable 


      
Namelist with Register No. 

      
Exam. Application Form

      

Syllabi (UG, PG &amp; Diploma)  

      
	  
	  
    



UG Non-Semester Programmes -  Part-1 Urudu and Sanskrit - Exam Timetable
MBA and MCA Semester Programmes -  Examination Timetable - April 2024
UG and PG Non-Semester Programmes -  Examination Timetable - April 2024

MCA Program - 2024 Syllabi- Subject Code details uploaded  


UG Non-Semester Programmes April 2024 Examination Notification (Tamil &amp; English)  

MBA and MCA April 2024 Exam Notification



UG - November 2023 Exam Results



I Semester MBA Program (2024 Calender Year batch) - Class Timetable 


I Semester MBA Program (2024 Calender Year batch) - Assignment Topics 


MBA AND MCA - November 2023 Exam Results Updated

PG - November 2023 Exam Results


MBA - IV Semester Project Coordinators List


MBA - IV Semester Project - Review Meeting Dates



I Year PG Arts Programs (2023 Academic Year batch) Trichy PCP - Class Timetable



I Year PG Science Programs (2023 Academic Year batch) Trichy PCP - Class Timetable

I Year PG (2023 Academic Year batch) - Class Timetable


III Semester MBA (2023 Calendar Year batch) - Assignment Topics


Calendar year 2022 (III Year), 2023 (II Year) - Tuition Fee Circular


MBA II, III and IV SEMESTER Fee Circular

1. MBA II SEMESTER CLASS TIME TABLE (ACADEMIC YEAR 2023-2025)
2. MBA II SEMESTER (ACADEMIC YEAR-2023-2025) ASSIGNMENT TOPICS
3. MBA III YEAR CLASS TIME TABLE (CALENDAR YEAR 2023)
4. MBA III SEMESTER (CALENDAR YEAR-2023) ASSIGNMENT TOPICS
5. MBA III SEMESTER ELECTIVE (CALENDAR YEAR 2023)
6.MBA IV YEAR CLASS TIME TABLE (ACADEMIC YEAR 2022-2024)
7. MBA IV SEMESTER (ACADEMIC YEAR-2022-2024) ASSIGNMENT TOPICS


UG and PG Non Semester, MBA, MCA and B.Ed. - November 2023 Exam Timetable 


II year and III Year UG - PCP Centres Class timetable. Click to view 



 

















 












    
	  
    
                            
    View all Notifications
    

      

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
            
            



          
        

      


      
April 2024 Exam Timetable 

      
April 2024 Exam Notification 

      
Nov&quot; , &quot;'&quot; , &quot;2023 Exam Results 

      
Class Timetable 


      
Namelist with Register No. 

      
Exam. Application Form

      

Syllabi (UG, PG &amp; Diploma)  

      
	  
	  
    



UG Non-Semester Programmes -  Part-1 Urudu and Sanskrit - Exam Timetable
MBA and MCA Semester Programmes -  Examination Timetable - April 2024
UG and PG Non-Semester Programmes -  Examination Timetable - April 2024

MCA Program - 2024 Syllabi- Subject Code details uploaded  


UG Non-Semester Programmes April 2024 Examination Notification (Tamil &amp; English)  

MBA and MCA April 2024 Exam Notification



UG - November 2023 Exam Results



I Semester MBA Program (2024 Calender Year batch) - Class Timetable 


I Semester MBA Program (2024 Calender Year batch) - Assignment Topics 


MBA AND MCA - November 2023 Exam Results Updated

PG - November 2023 Exam Results


MBA - IV Semester Project Coordinators List


MBA - IV Semester Project - Review Meeting Dates



I Year PG Arts Programs (2023 Academic Year batch) Trichy PCP - Class Timetable



I Year PG Science Programs (2023 Academic Year batch) Trichy PCP - Class Timetable

I Year PG (2023 Academic Year batch) - Class Timetable


III Semester MBA (2023 Calendar Year batch) - Assignment Topics


Calendar year 2022 (III Year), 2023 (II Year) - Tuition Fee Circular


MBA II, III and IV SEMESTER Fee Circular

1. MBA II SEMESTER CLASS TIME TABLE (ACADEMIC YEAR 2023-2025)
2. MBA II SEMESTER (ACADEMIC YEAR-2023-2025) ASSIGNMENT TOPICS
3. MBA III YEAR CLASS TIME TABLE (CALENDAR YEAR 2023)
4. MBA III SEMESTER (CALENDAR YEAR-2023) ASSIGNMENT TOPICS
5. MBA III SEMESTER ELECTIVE (CALENDAR YEAR 2023)
6.MBA IV YEAR CLASS TIME TABLE (ACADEMIC YEAR 2022-2024)
7. MBA IV SEMESTER (ACADEMIC YEAR-2022-2024) ASSIGNMENT TOPICS


UG and PG Non Semester, MBA, MCA and B.Ed. - November 2023 Exam Timetable 


II year and III Year UG - PCP Centres Class timetable. Click to view 



 

















 












    
	  
    
                            
    View all Notifications
    

      

      &quot;))]</value>
      <webElementGuid>a5e26c63-9c4d-4e7e-87bf-cb41147c9a1b</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
