<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_(function()function c()var ba.contentD_92d7c6</name>
   <tag></tag>
   <elementGuidId>67d4d27b-3b41-4820-9890-dca48170d865</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.slBg</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>4cf0adca-2512-4f60-82be-c5535c2d2e1f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>slBg</value>
      <webElementGuid>f2b5ae3d-bc67-4dc5-a59a-c009d3cd34c7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    

    
    
    
    
    
(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement('script');d.innerHTML=&quot;window.__CF$cv$params={r:'9079467c48b55957',t:'MTczNzgxODcyMS4wMDAwMDA='};var a=document.createElement('script');a.nonce='';a.src='/cdn-cgi/challenge-platform/scripts/jsd/main.js';document.getElementsByTagName('head')[0].appendChild(a);&quot;;b.getElementsByTagName('head')[0].appendChild(d)}}if(document.body){var a=document.createElement('iframe');a.height=1;a.width=1;a.style.position='absolute';a.style.top=0;a.style.left=0;a.style.border='none';a.style.visibility='hidden';document.body.appendChild(a);if('loading'!==document.readyState)c();else if(window.addEventListener)document.addEventListener('DOMContentLoaded',c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);'loading'!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();



    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute('value', d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute('value', -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $('#full-overlay').show();

    }

    function checkOldCookie()
    {
        if (!getCookie('HubClientLogin'))
        {
            var oldCookie = getCookie('ClientLogin');
            if (oldCookie)
            {
                $('#Client_CorpCode').val(getKeyValue(oldCookie, 'CorpCode'));
                $('#Client_LocationCode').val(getKeyValue(oldCookie, 'LocationCode'));
                $('#Client_UserName').val(getKeyValue(oldCookie, 'UserName'));
            }

        }

        if (!getCookie('HubCorpLogin'))
        {
            var oldCookie = getCookie('CorpLogin');
            if (oldCookie)
            {
                $('#Corp_CorpCode').val(getKeyValue(oldCookie, 'CorpCode'));
                $('#Corp_UserName').val(getKeyValue(oldCookie, 'UserName'));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(';');
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == ' ')
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += ';domain=' + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split('&amp;');
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split('=');
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_CorpCode&quot;)</value>
      <webElementGuid>28825b24-bbc0-447c-b891-9a240f8230d5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[@class=&quot;slBg&quot;]</value>
      <webElementGuid>08c93cf5-efa5-4407-b4ed-c78821869eaf</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>0b26ae55-beee-4f13-8912-f93d4768cb75</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    

    
    
    
    
    
(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);d.innerHTML=&quot;window.__CF$cv$params={r:&quot; , &quot;'&quot; , &quot;9079467c48b55957&quot; , &quot;'&quot; , &quot;,t:&quot; , &quot;'&quot; , &quot;MTczNzgxODcyMS4wMDAwMDA=&quot; , &quot;'&quot; , &quot;};var a=document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);a.nonce=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;a.src=&quot; , &quot;'&quot; , &quot;/cdn-cgi/challenge-platform/scripts/jsd/main.js&quot; , &quot;'&quot; , &quot;;document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(a);&quot;;b.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(d)}}if(document.body){var a=document.createElement(&quot; , &quot;'&quot; , &quot;iframe&quot; , &quot;'&quot; , &quot;);a.height=1;a.width=1;a.style.position=&quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;a.style.top=0;a.style.left=0;a.style.border=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;a.style.visibility=&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;document.body.appendChild(a);if(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState)c();else if(window.addEventListener)document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;,c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();



    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $(&quot; , &quot;'&quot; , &quot;#full-overlay&quot; , &quot;'&quot; , &quot;).show();

    }

    function checkOldCookie()
    {
        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubClientLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;ClientLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Client_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_LocationCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;LocationCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }

        }

        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubCorpLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;CorpLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Corp_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Corp_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += &quot; , &quot;'&quot; , &quot;;domain=&quot; , &quot;'&quot; , &quot; + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split(&quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;);
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_CorpCode&quot;)&quot;) or . = concat(&quot;
    

    
    
    
    
    
(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);d.innerHTML=&quot;window.__CF$cv$params={r:&quot; , &quot;'&quot; , &quot;9079467c48b55957&quot; , &quot;'&quot; , &quot;,t:&quot; , &quot;'&quot; , &quot;MTczNzgxODcyMS4wMDAwMDA=&quot; , &quot;'&quot; , &quot;};var a=document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);a.nonce=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;a.src=&quot; , &quot;'&quot; , &quot;/cdn-cgi/challenge-platform/scripts/jsd/main.js&quot; , &quot;'&quot; , &quot;;document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(a);&quot;;b.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(d)}}if(document.body){var a=document.createElement(&quot; , &quot;'&quot; , &quot;iframe&quot; , &quot;'&quot; , &quot;);a.height=1;a.width=1;a.style.position=&quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;a.style.top=0;a.style.left=0;a.style.border=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;a.style.visibility=&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;document.body.appendChild(a);if(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState)c();else if(window.addEventListener)document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;,c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();



    








    



    
        
            Your session is about to expire. Click Continue to extend your session.
            
                Continue
            
        
    








    
        
        SiteLink myHub
    
    Smart Management Software for Self-Storage
    Visit the SiteLink myHub information page  to learn more. For a Demo login, contact Sales.
    
    

        
            

    
        
            SiteLink Web Edition
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Client_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, d.getTimezoneOffset());
            

            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    Location Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
            
        
        
            
        
    

        

        

            
                
    
        
            Corporate Control Center
        
        
            
            
            
                var d = new Date();
                document.getElementById(&quot;Corp_GmtOffset&quot;).setAttribute(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, -d.getTimezoneOffset() / 60);
            
            
                
                    
                
                
                    Corp Code
                    
                        
                        
                    
                
                
                    User Name
                    
                        
                        
                    
                
                
                    Password
                    
                        
                        
                    
                
                
                     
                    
                         
                    
                
            
        
        
            
        
    

            
    
    
        
            
                
                
                
                
                
            
        
    


    window.onbeforeunload = function()
    {
        ShowLoader();
    }

    $(document).ready(function()
    {
        checkOldCookie();
    });

    function ShowLoader()
    {
        $(&quot; , &quot;'&quot; , &quot;#full-overlay&quot; , &quot;'&quot; , &quot;).show();

    }

    function checkOldCookie()
    {
        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubClientLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;ClientLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Client_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_LocationCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;LocationCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Client_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }

        }

        if (!getCookie(&quot; , &quot;'&quot; , &quot;HubCorpLogin&quot; , &quot;'&quot; , &quot;))
        {
            var oldCookie = getCookie(&quot; , &quot;'&quot; , &quot;CorpLogin&quot; , &quot;'&quot; , &quot;);
            if (oldCookie)
            {
                $(&quot; , &quot;'&quot; , &quot;#Corp_CorpCode&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;CorpCode&quot; , &quot;'&quot; , &quot;));
                $(&quot; , &quot;'&quot; , &quot;#Corp_UserName&quot; , &quot;'&quot; , &quot;).val(getKeyValue(oldCookie, &quot; , &quot;'&quot; , &quot;UserName&quot; , &quot;'&quot; , &quot;));
            }
        }
    }

    function getCookie(cname)
    {
        var name = cname + &quot;=&quot;;
        var decodedCookie = decodeURIComponent(document.cookie);
        var ca = decodedCookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; ca.length; i++)
        {
            var c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)
            {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0)
            {
                return c.substring(name.length, c.length);
            }
        }
        return &quot;&quot;;
    }

    function setCookie(cname, cvalue, days, domain)
    {
        var d = new Date();
        d.setDate(d.getDate() + days);
        var expires = &quot;expires=&quot; + d.toUTCString();
        var cookieString = cname + &quot;=&quot; + cvalue + &quot;;&quot; + &quot;SameSite=None; Secure;&quot; + expires + &quot;;path=/&quot;;
        if (domain)
            cookieString += &quot; , &quot;'&quot; , &quot;;domain=&quot; , &quot;'&quot; , &quot; + domain;
        document.cookie = cookieString;
    }

    function getKeyValue(cookieString, Key)
    {
        var split = cookieString.split(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;);
        for (var i = 0; i &lt; split.length ; i++)
        {
            var keyValuePair = split[i].split(&quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;);
            if (keyValuePair.length == 2 &amp;&amp; keyValuePair[0] == Key)
            {
                return keyValuePair[1];
            }
        }
        return &quot;&quot;;
    }


    
        
            
                © 2025 SiteLink Software, LLC.  | 
                Terms and Conditions  | Privacy Policy
            
            
        
    



    


id(&quot;Client_CorpCode&quot;)&quot;))]</value>
      <webElementGuid>97ebbe0c-3c70-4190-9baf-ac840b3dfc6f</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
